//! Desktop UI (`kana gui`): the same sessions as the terminal, with illustrated cards.

use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::NaiveDate;
use eframe::egui::{
    self, Align, Color32, FontData, FontDefinitions, FontFamily, Key, Layout, RichText, Vec2,
};

use crate::card::{self, Card, Outcome};
use crate::images;
use crate::kana::{Kana, Script};
use crate::session::{FAST, Mode, Session, Summary};
use crate::storage;

const FONT: &[u8] = include_bytes!("../assets/fonts/KleeOne-Kana.ttf");
/// A right answer stays on screen this long before the next card.
const AUTO_NEXT: Duration = Duration::from_millis(900);
const GREEN: Color32 = Color32::from_rgb(70, 150, 90);
const RED: Color32 = Color32::from_rgb(200, 80, 70);

pub fn run() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Kana")
            .with_inner_size([520.0, 760.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Kana",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_fonts(fonts());
            Ok(Box::new(App::new()))
        }),
    )
}

/// Klee One for kana (its own family, and as a fallback for kana inside French text).
fn fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    fonts
        .font_data
        .insert("klee".into(), Arc::new(FontData::from_static(FONT)));
    fonts
        .families
        .insert(FontFamily::Name("kana".into()), vec!["klee".into()]);
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .push("klee".into());
    fonts
}

fn kana_text(text: &str, size: f32) -> RichText {
    RichText::new(text)
        .size(size)
        .family(FontFamily::Name("kana".into()))
        .strong()
}

fn card_image(script: Script, id: usize, size: f32) -> egui::Image<'static> {
    let uri = format!(
        "bytes://{}/{}.svg",
        script.name(),
        script.table()[id].romaji[0]
    );
    egui::Image::from_bytes(uri, images::card(script, id))
        .fit_to_exact_size(Vec2::splat(size))
        .corner_radius(12)
}

enum Screen {
    Home,
    Review(Box<Review>),
    Done(Summary),
    Gallery(Option<usize>), // the selected card's id
}

struct Review {
    session: Session,
    current: Option<Question>,
}

struct Question {
    x: usize, // index in `cards`
    step: Step,
    answer: String,
    shown_at: Instant,
}

enum Step {
    Intro,    // a new kana: show its card first
    Asking,   // waiting for the answer
    Revealed, // inverse mode: the kana is shown, the user grades themselves
    Feedback {
        outcome: Outcome,
        at: Instant,
        elapsed: Duration,
    },
}

struct App {
    script: Script,
    mode: Mode,
    today: NaiveDate,
    cards: Vec<Card>,
    screen: Screen,
    notice: Option<String>,
    load_error: Option<String>,
}

impl App {
    fn new() -> App {
        let mut app = App {
            script: Script::Hiragana,
            mode: Mode::Normal,
            today: chrono::Local::now().date_naive(),
            cards: Vec::new(),
            screen: Screen::Home,
            notice: None,
            load_error: None,
        };
        app.load();
        app
    }

    /// Loads the deck of the current script; a corrupt file blocks reviews instead of being overwritten.
    fn load(&mut self) {
        match storage::load(self.script) {
            Ok(cards) => {
                self.cards = cards;
                self.load_error = None;
            }
            Err(e) => {
                self.cards = Vec::new();
                self.load_error = Some(format!(
                    "Impossible de charger la progression : {e}\n(« kana reset » repart de zéro.)"
                ));
            }
        }
    }

    fn save(&mut self) {
        if let Err(e) = storage::save(self.script, &self.cards) {
            self.notice = Some(format!("Erreur de sauvegarde : {e}"));
        }
    }

    fn start(&mut self, ahead: bool) {
        self.notice = None;
        if let Some(group) =
            card::unlock_next_group(&mut self.cards, self.script.table(), self.today)
        {
            let new_kana: Vec<&str> = self
                .script
                .table()
                .iter()
                .filter(|k| k.group == group)
                .map(|k| k.kana)
                .collect();
            self.notice = Some(format!("Nouveau groupe débloqué : {}", new_kana.join(" ")));
            self.save();
        }
        let session = Session::new(&self.cards, self.script, ahead);
        self.screen = Screen::Review(Box::new(Review {
            session,
            current: None,
        }));
    }

    fn kana(&self, x: usize) -> &'static Kana {
        &self.script.table()[self.cards[x].id]
    }

    fn home(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(12.0);
            ui.label(kana_text("かな", 64.0));
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let before = self.script;
                ui.selectable_value(&mut self.script, Script::Hiragana, "Hiragana");
                ui.selectable_value(&mut self.script, Script::Katakana, "Katakana");
                if self.script != before {
                    self.load();
                }
                ui.separator();
                ui.selectable_value(&mut self.mode, Mode::Normal, "Kana → romaji");
                ui.selectable_value(&mut self.mode, Mode::Inverse, "Romaji → kana");
            });
        });
        ui.add_space(16.0);

        if let Some(error) = &self.load_error {
            ui.colored_label(RED, error);
            return;
        }
        if let Some(notice) = &self.notice {
            ui.label(RichText::new(notice).strong());
            ui.add_space(8.0);
        }

        let due = self.cards.iter().filter(|c| c.is_due(self.today)).count();
        ui.label(format!(
            "Kana débloqués : {}/{}",
            self.cards.len(),
            self.script.table().len()
        ));
        ui.label(RichText::new(format!("À réviser aujourd'hui : {due}")).strong());
        ui.add_space(8.0);
        ui.label("Mémorisation (stabilité) :");
        let total = self.cards.len().max(1) as f32;
        for (label, n) in card::stability_buckets(&self.cards) {
            ui.horizontal(|ui| {
                ui.add_sized([70.0, 18.0], egui::Label::new(label));
                ui.add(egui::ProgressBar::new(n as f32 / total).text(n.to_string()));
            });
        }
        ui.add_space(20.0);

        // A fresh deck has nothing due, but starting a session unlocks group 1.
        let can_review = due > 0 || self.cards.is_empty();
        ui.vertical_centered(|ui| {
            let button = egui::Button::new(RichText::new("Réviser").size(20.0))
                .min_size(Vec2::new(200.0, 44.0));
            if ui.add_enabled(can_review, button).clicked() {
                self.start(false);
            }
            if !can_review {
                ui.label("Rien à réviser pour l'instant.");
            }
            ui.add_space(6.0);
            if ui
                .add_enabled(
                    !self.cards.is_empty(),
                    egui::Button::new("Réviser en avance").min_size(Vec2::new(200.0, 32.0)),
                )
                .on_hover_text("Toutes les cartes débloquées, même celles pas encore dues")
                .clicked()
            {
                self.start(true);
            }
            ui.add_space(6.0);
            if ui
                .add(egui::Button::new("Galerie").min_size(Vec2::new(200.0, 32.0)))
                .clicked()
            {
                self.screen = Screen::Gallery(None);
            }
        });
    }

    fn review(&mut self, ui: &mut egui::Ui, mut review: Box<Review>) -> Screen {
        if ui.button("← Arrêter").clicked() {
            return Screen::Done(review.session.finish(&self.cards));
        }
        if let Some(notice) = &self.notice {
            ui.vertical_centered(|ui| ui.label(RichText::new(notice).strong()));
        }

        let question = match review.current.take() {
            Some(q) => q,
            None => match review.session.next(&self.cards, self.today) {
                Some(x) => Question {
                    x,
                    step: if self.cards[x].memory.is_none() {
                        Step::Intro
                    } else {
                        Step::Asking
                    },
                    answer: String::new(),
                    shown_at: Instant::now(),
                },
                None => return Screen::Done(review.session.finish(&self.cards)),
            },
        };
        review.current = self.question(ui, &mut review.session, question);
        Screen::Review(review)
    }

    /// Shows one question. Returns it to keep it on screen, or None to move on.
    fn question(
        &mut self,
        ui: &mut egui::Ui,
        session: &mut Session,
        mut q: Question,
    ) -> Option<Question> {
        let k = self.kana(q.x);
        let script = self.script;
        let id = self.cards[q.x].id;
        let enter = ui.ctx().input(|i| i.key_pressed(Key::Enter));
        let mut outcome = None;

        ui.vertical_centered(|ui| {
            ui.add_space(12.0);
            match q.step {
                Step::Intro => {
                    ui.label(RichText::new("Nouveau kana").strong());
                    ui.add(card_image(script, id, 300.0));
                    ui.label(kana_text(
                        &format!("{}  =  {}", k.kana, k.romaji.join(" / ")),
                        32.0,
                    ));
                    ui.add_space(4.0);
                    ui.label(RichText::new(k.mnemonic).italics().size(16.0));
                    ui.add_space(12.0);
                    if ui.button("Compris").clicked() || enter {
                        q.step = Step::Asking;
                        q.shown_at = Instant::now();
                    }
                }
                Step::Asking => match self.mode {
                    Mode::Normal => {
                        ui.label(kana_text(k.kana, 160.0));
                        ui.label("Romaji ?");
                        let response = ui.add(
                            egui::TextEdit::singleline(&mut q.answer)
                                .desired_width(160.0)
                                .font(egui::TextStyle::Heading)
                                .align(egui::Align2::CENTER_CENTER),
                        );
                        if response.lost_focus() && enter {
                            let answer = q.answer.trim().to_lowercase();
                            if !answer.is_empty() {
                                let elapsed = q.shown_at.elapsed();
                                outcome = Some(if !k.romaji.contains(&answer.as_str()) {
                                    Outcome::Wrong
                                } else if elapsed < FAST {
                                    Outcome::Fast
                                } else {
                                    Outcome::Correct
                                });
                            }
                        }
                        response.request_focus();
                    }
                    Mode::Inverse => {
                        ui.label(RichText::new(k.romaji[0]).size(96.0));
                        ui.label("Écris le kana sur papier.");
                        ui.add_space(12.0);
                        if ui.button("Voir la réponse").clicked() || enter {
                            q.step = Step::Revealed;
                        }
                    }
                },
                Step::Revealed => {
                    ui.label(RichText::new(k.romaji[0]).size(48.0));
                    ui.label(kana_text(k.kana, 160.0));
                    ui.label("C'était juste ?");
                    ui.horizontal(|ui| {
                        ui.add_space(ui.available_width() / 2.0 - 90.0);
                        if ui.button(RichText::new("Juste (o)").color(GREEN)).clicked()
                            || ui.ctx().input(|i| i.key_pressed(Key::O))
                        {
                            outcome = Some(Outcome::Correct);
                        }
                        if ui.button(RichText::new("Raté (n)").color(RED)).clicked()
                            || ui.ctx().input(|i| i.key_pressed(Key::N))
                        {
                            outcome = Some(Outcome::Wrong);
                        }
                    });
                }
                Step::Feedback {
                    outcome: result,
                    at,
                    elapsed,
                } => {
                    if result == Outcome::Wrong {
                        ui.label(RichText::new("Raté !").color(RED).size(28.0).strong());
                        ui.add(card_image(script, id, 260.0));
                        ui.label(kana_text(
                            &format!("{}  =  {}", k.kana, k.romaji.join(" / ")),
                            32.0,
                        ));
                        ui.label(RichText::new(k.mnemonic).italics().size(16.0));
                        ui.add_space(12.0);
                        if ui.button("Continuer").clicked() || enter {
                            return None;
                        }
                    } else {
                        ui.label(kana_text(k.kana, 160.0));
                        let text = if result == Outcome::Fast {
                            "Correct, et rapide !"
                        } else {
                            "Correct !"
                        };
                        ui.label(RichText::new(text).color(GREEN).size(28.0).strong());
                        if self.mode == Mode::Normal {
                            ui.label(format!("{:.1} s", elapsed.as_secs_f32()));
                        }
                        if at.elapsed() >= AUTO_NEXT || enter {
                            return None;
                        }
                        ui.ctx().request_repaint_after(AUTO_NEXT);
                    }
                }
            }
            Some(())
        })
        .inner?;

        if let Some(outcome) = outcome {
            session.record(&mut self.cards, q.x, outcome, self.today);
            self.save();
            q.step = Step::Feedback {
                outcome,
                at: Instant::now(),
                elapsed: q.shown_at.elapsed(),
            };
        }
        Some(q)
    }

    fn done(&mut self, ui: &mut egui::Ui, summary: &Summary) -> bool {
        let mut home = false;
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label(RichText::new("Session terminée !").size(28.0).strong());
            ui.add_space(16.0);
            ui.label(
                RichText::new(format!("Score : {}/{}", summary.correct, summary.asked)).size(20.0),
            );
            ui.label(format!("Cartes renforcées : {}", summary.strengthened));
            ui.label(match summary.next_review {
                Some(date) => format!("Prochaine révision : {}", date.format("%d/%m/%Y")),
                None => "Prochaine révision : aucune".to_string(),
            });
            ui.add_space(24.0);
            home = ui.button("Accueil").clicked() || ui.ctx().input(|i| i.key_pressed(Key::Enter));
        });
        home
    }

    fn gallery(&mut self, ui: &mut egui::Ui, selected: &mut Option<usize>) -> bool {
        let back = ui.button("← Accueil").clicked();
        if self.cards.is_empty() {
            ui.label("Aucun kana débloqué. Lance une révision pour commencer.");
            return back;
        }
        let script = self.script;
        if let Some(id) = *selected {
            let k = &script.table()[id];
            ui.vertical_centered(|ui| {
                ui.add(card_image(script, id, 280.0));
                ui.label(kana_text(
                    &format!("{}  =  {}", k.kana, k.romaji.join(" / ")),
                    28.0,
                ));
                ui.label(RichText::new(k.mnemonic).italics().size(15.0));
            });
            ui.separator();
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(
                Layout::left_to_right(Align::Min).with_main_wrap(true),
                |ui| {
                    let mut ids: Vec<usize> = self.cards.iter().map(|c| c.id).collect();
                    ids.sort();
                    for id in ids {
                        let image = card_image(script, id, 88.0).sense(egui::Sense::click());
                        if ui.add(image).clicked() {
                            *selected = Some(id);
                        }
                    }
                },
            );
        });
        back
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default_margins().show(ui, |ui| {
            let screen = std::mem::replace(&mut self.screen, Screen::Home);
            self.screen = match screen {
                Screen::Home => {
                    self.home(ui);
                    // `home` may have switched screens itself.
                    std::mem::replace(&mut self.screen, Screen::Home)
                }
                Screen::Review(review) => self.review(ui, review),
                Screen::Done(summary) => {
                    if self.done(ui, &summary) {
                        self.notice = None;
                        Screen::Home
                    } else {
                        Screen::Done(summary)
                    }
                }
                Screen::Gallery(mut selected) => {
                    if self.gallery(ui, &mut selected) {
                        Screen::Home
                    } else {
                        Screen::Gallery(selected)
                    }
                }
            };
        });
    }
}
