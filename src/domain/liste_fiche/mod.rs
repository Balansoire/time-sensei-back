mod generate;

use std::cmp::Ordering;

use crate::domain::revision::{FicheRevision};

pub use generate::generate;

pub type ListeFiche = Vec<FicheRevision>;

pub trait ListeRevision {
    fn nouvelle_revision(&self) -> usize;
    fn revision_groupe(&self, groupe: &mut Vec<usize>) -> usize;
    fn groupe_de_revision(&self, groupe: &String) -> Vec<usize>;
    fn resultat_revision(&mut self, index: usize, resultat: bool);
    fn resultat_revision_groupe(&mut self, groupe: &mut Vec<usize>, index: usize, resultat: bool);
}
impl ListeRevision for ListeFiche {
    fn nouvelle_revision(&self) -> usize {
        0
    }

    fn revision_groupe(&self, groupe: &mut Vec<usize>) -> usize {
        groupe.remove(0)
    }
    
    fn groupe_de_revision(&self, groupe: &String) -> Vec<usize> {
        let mut indexes = Vec::new();

        for i in 0..self.len() {
            if self.get(i).unwrap().get_groupe().eq(groupe) {
                indexes.push(i);
            }
        }
        indexes
    }

    fn resultat_revision(&mut self, index: usize, resultat: bool) {
        let mut fiche = self.remove(index);

        if resultat {
            fiche.reussite_revision();
        } else {
            fiche.echec_revision();
        }

        let mut n = self.len();
        for k in self.iter().rev() {
            if Ordering::Less == fiche.cmp(k) {
                n = n.saturating_sub(1);
            } else {
                self.insert(n, fiche);
                break;
            }
        }
    }

    fn resultat_revision_groupe(&mut self, groupe: &mut Vec<usize>, index: usize, resultat: bool) {
         let mut fiche = self.remove(index);

        if resultat {
            fiche.reussite_revision();
        } else {
            fiche.echec_revision();
        }
         
         let mut n = self.len();
         for k in self.iter().rev() {
            if Ordering::Less == fiche.cmp(k) {
                n = n.saturating_sub(1);
            } else {
                self.insert(n, fiche);
                break;
                }
            }
        
            for i in groupe {
                if *i <= n {
                    *i -= 1;
                }
         }
    }
}
