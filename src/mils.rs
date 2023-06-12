use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};

use crate::matrix_csr::Matrix;

impl Matrix {
    // Main code for MILS
    pub fn mils(&mut self, n: &usize, k: &usize) {
        let mut nivel: usize = 1;
        let mut iter_n: usize = 0;
        let mut iter_k: usize = 0;
        let mut bw_0 = self.bandwidth();
        let mut h: HashMap<usize, HashSet<usize>> = HashMap::new();
        let original_labels = self.labels.clone();
        let mut mils_labels: Vec<usize>;

        self.local_search();
        while (iter_n < *n && (bw_0 > self.min_bw) && iter_k < *k) {
            // dbg!(&h);
            self.perturbation(nivel, &mut h);
            self.local_search();
            if (self.bw < bw_0) {
                bw_0 = self.bw;
                iter_n = 0;
                nivel = 1;
            } else {
                nivel += 1;
                iter_n += 1;
                if iter_n > n / 20 {
                    // reiniciar h
                    h = HashMap::new();
                    // TODO: verificar necessidade
                    mils_labels = self.labels.clone();
                    let p = self.pseudo_george_liu(self.labels[0]);
                    self.labels = original_labels.clone();
                    self.cmr_labels(p);
                    self.bandwidth();
                    if (self.bw < bw_0) {
                        bw_0 = self.bw;
                        iter_n = 0;
                        nivel = 1;
                    } else {
                        self.labels = mils_labels;
                    }
                }
            }
            iter_k += 1;
        }
    }

    fn perturbation(&mut self, nivel: usize, h: &mut HashMap<usize, HashSet<usize>>) {
        let mut iter = 1;
        let criticos = self.criticals();
        let options: HashSet<usize> = HashSet::from_iter(self.labels.iter().cloned());
        while (iter <= nivel) {
            let v = criticos.choose(&mut rand::thread_rng()).unwrap();
            let mut u: &usize = v;
            if let Some(n_v) = h.get_mut(v) {
                // Vec of vertices not in history of v
                let n_v_copy = n_v.clone();
                let diff: Vec<&usize> = options.symmetric_difference(&n_v_copy).collect();
                let u = *diff.choose(&mut rand::thread_rng()).unwrap_or(&v);
                n_v.insert(*u);
                /*print!("v= {};", v);
                print!("u={:?}; ", *u);
                print!("o={:?}; ", &options);
                println!("diff={:?}", diff);
                dbg!(&h);*/
            } else {
                let u = self.labels.choose(&mut rand::thread_rng()).unwrap();
                h.insert(*v, HashSet::from([*u]));
                /* println!("\t primeiro");
                dbg!(&h); */
            }
            self.labels.swap(*v, *u);
            iter += 1;
        }
    }

    // Proceeds with local search and change labels if a better labeling if found
    fn local_search(&mut self) {
        let criticos = self.criticals();
        let mut bw_0 = self.bandwidth();
        for v in criticos {
            for u in self.neighbour_of_criticals2(v) {
                // println!("O{:?}", &self.labels);
                self.labels.swap(v, u);
                if self.bandwidth() <= bw_0 {
                    bw_0 = self.bw;
                    // println!("MELHOROU! {}", bw_0);
                    // println!("M{:?}", &self.labels);
                } else {
                    // Return to previous situation
                    self.labels.swap(v, u);
                    self.bw = bw_0;
                    // println!("V{:?}", &self.labels);
                }
            }
        }
    }

    // Middle of rotulations of v
    fn mid(&self, v: usize, neighbour: &mut &[usize]) -> usize {
        // mid(v) = ⌊(max(v) + min(v))/2⌋
        let mut neighbour: Vec<usize> = neighbour.to_vec();
        neighbour.sort_unstable();
        let min = neighbour.first().unwrap_or(&0);
        let max = neighbour.last().unwrap_or(&0);
        (min + max) / 2
    }

    fn neighbour_of_criticals2(&self, v: usize) -> Vec<usize> {
        let mut neighbour = self.get_columns_of_row(self.old_label(v)).clone();
        let mid_v = self.mid(v, &mut neighbour);
        let mut neighbour_of_criticals: Vec<usize> = Vec::new();
        // dbg!(v, neighbour, mid_v);
        for u in neighbour {
            let u = self.labels[*u];
            neighbour_of_criticals.push(u);
            neighbour_of_criticals.push(self.mid(u, &mut neighbour));
        }
        neighbour_of_criticals.extend(neighbour);
        neighbour_of_criticals
    }

    fn neighbour_of_criticals(&self, v: usize) -> Vec<usize> {
        // TODO: Ordenar em ordem crescente  do valor |mid(v) − f (u)|
        /*
        Trocar o rótulo de cada vértice crítico com os rótulos dos vértices u ∈ N ′ (v), em ordem crescente do valor |mid(v) − f (u)| até que se encontre uma solução melhor.
        O conjunto de candidatos a serem trocados com o vértice v é definido por
        N′(v) = {u : |mid(v) − f (u)| < |mid(v) − f (v)|} e
        mid(v) = ⌊(max(v) + min(v))/2⌋.
         */
        let mut neighbour_of_criticals: Vec<usize> = Vec::with_capacity(self.degree(v));
        let mut neighbour = self.get_columns_of_row(self.old_label(v)).clone();
        let mid_v = self.mid(v, &mut neighbour);
        // dbg!(v, neighbour, mid_v);
        for u in neighbour {
            let u = self.labels[*u];
            if self.mid(v, &mut neighbour).abs_diff(u) <= self.mid(v, &mut neighbour).abs_diff(v) {
                // dbg!(v, u, mid_v);
                // println!("{:?} < {:?}", self.mid(v, &mut neighbour).abs_diff(u), self.mid(v, &mut neighbour).abs_diff(v));
                neighbour_of_criticals.push(u);
            }
        }
        // dbg!(&neighbour_of_criticals);
        neighbour_of_criticals.extend(neighbour);
        // dbg!(&neighbour_of_criticals);
        neighbour_of_criticals
    }

    // println!("\tcriticals = {:?}", self.criticals);
    // let mut rng = rand::thread_rng();
    // let v: usize = rng.gen_range(1..self.m);
}
