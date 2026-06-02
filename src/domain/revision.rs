use std::cmp::{min, Ordering};

use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Kana {
  pub romaji: String,
  pub kana: String,
  pub groupe: String,
  pub sous_groupe: String,
}

impl Kana {
  pub fn new(r: &str, k: &str, g: &str, sg: &str) -> Self {
      Self {
          romaji: r.into(),
          kana: k.into(),
          groupe: g.into(),
          sous_groupe: sg.into(),
      }
  }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Kanji {
  pub romaji_l: String,
  pub kana_l: String,

  pub romaji_c: String,
  pub kana_c: String,

  pub kanji: String,

  pub niveau: String,
  pub sous_groupe: String,
  pub signification: String,
}

impl Kanji {
  pub fn new(rl: &str, kl: &str, rc: &str, kc: &str, k: &str, g: &str, n: &str, s: &str) -> Self {
      Self {
          romaji_l: rl.into(),
          kana_l: kl.into(),
          romaji_c: rc.into(),
          kana_c: kc.into(),
          kanji: k.into(),
          niveau: n.into(),
          sous_groupe: g.into(),
          signification: s.into(),
      }
  }
}

#[derive(Clone, Deserialize, Serialize)]
pub enum Caractere {
  Hiragana(Kana),
  Katakana(Kana),
  Kanji(Kanji),
}

#[derive(Clone, Deserialize, Serialize)]
pub struct FicheRevision {
  caractere: Caractere,

  suite_succes: usize,
  derniere_revision: NaiveDate,
  nombre_revisions: usize,
  nombre_succes: usize,
}


impl FicheRevision {
  pub fn new(c: Caractere) -> Self{
    Self {
      caractere: c,

      suite_succes: 0,
      derniere_revision: Utc::now().naive_local().date(),
      nombre_revisions: 0,
      nombre_succes: 0,
    }
  }

  pub fn new_hiragana(r: &str, k: &str, g: &str, sg: &str) -> Self {
    Self::new(Caractere::Hiragana(Kana::new(r, k, g, sg)))
  }

  pub fn new_katakana(r: &str, k: &str, g: &str, sg: &str) -> Self {
    Self::new(Caractere::Hiragana(Kana::new(r, k, g, sg)))
  }

  pub fn new_kanji(rl: &str, kl: &str, rc: &str, kc: &str, k: &str, g: &str, n: &str, s: &str) -> Self {
    Self::new(Caractere::Kanji(Kanji::new(rl, kl, rc, kc, k, g, n, s)))
  }

    pub fn get_caractere(&self) -> &String {
      match &self.caractere {
        Caractere::Hiragana(kana) => &kana.kana,
        Caractere::Katakana(kana) => &kana.kana,
        Caractere::Kanji(kanji) => &kanji.kanji,
      }
    }

    pub fn get_groupe(&self) -> &String {
      match &self.caractere {
        Caractere::Hiragana(kana) => &kana.groupe,
        Caractere::Katakana(kana) => &kana.groupe,
        Caractere::Kanji(kanji) => &kanji.niveau,
      }
    }

    pub fn get_sous_groupe(&self) -> &String {
      match &self.caractere {
        Caractere::Hiragana(kana) => &kana.sous_groupe,
        Caractere::Katakana(kana) => &kana.sous_groupe,
        Caractere::Kanji(kanji) => &kanji.sous_groupe,
      }
    }

    pub fn get_nombre_revisions(&self) -> usize {
      self.nombre_revisions
    }

    pub fn get_nombre_succes(&self) -> usize {
      self.nombre_succes
    }

  fn actualiser_date(&mut self) {
    self.derniere_revision = Utc::now().naive_local().date()
  }

  pub fn echec_revision(&mut self) {
    self.suite_succes = if Utc::now().naive_local().date() == self.derniere_revision {
        min(0, self.suite_succes - 1)
    } else {
        0
    };

    self.actualiser_date();
  }

  pub fn reussite_revision(&mut self) {
    if Utc::now().naive_local().date() != self.derniere_revision {
        self.suite_succes += 1;
    };

    self.actualiser_date();
  }

  pub  fn cmp(&self, other: &Self) -> Ordering {
        let delta = self.derniere_revision.signed_duration_since(other.derniere_revision).num_days()
                + self.suite_succes as i64
                - other.suite_succes as i64;
        if delta < 0 {
          return Ordering::Less;
        }
        if delta > 0 {
          return Ordering::Greater
        }
        Ordering::Equal
    }
}