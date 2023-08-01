use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

use crate::matrix_csr::Matrix;

impl Matrix {
    // Main code for metaheuristic MILS
    #[inline(always)]
    pub fn mils(&mut self, n: &usize, k: &usize) {
        let mut nivel: usize = 0;
        let mut iter_n: usize = 0;
        let mut iter_k: usize = 0;
        let mut bw_0 = self.bw;
        let mut history: HashMap<usize, HashSet<usize>> = HashMap::new();
        // Reinicialize with different pseudo-peripheral vertex
        let mut h_restart: HashSet<usize> = HashSet::new();
        let original_labels = self.labels.clone();
        let mut pseudo: usize;
        let options: HashSet<usize> = HashSet::from_iter(self.labels.iter().cloned());

        self.local_search();
        while iter_n < *n { //&& (bw_0 > self.min_bw)
            // dbg!(n - nivel);
            self.perturbation(n - nivel, &mut history, &options);
            self.local_search();
            if self.bw < bw_0 {
                bw_0 = self.bw;
                iter_n = 0;
                nivel = 0;
            } else {
                nivel += 1;
                iter_n += 1;
            }

            // Greed restart RCM-GL when iter_n == n
            if iter_n == *n && iter_k < *k {
                let diff: HashSet<&usize> = options.difference(&h_restart).collect();
                let diff: Vec<&usize> = diff.into_iter().collect();
                pseudo = **diff
                    .choose(&mut rand::thread_rng())
                    .unwrap_or(&&self.labels[0]);
                h_restart.insert(pseudo);
                let p = self.pseudo_george_liu(pseudo);
                self.labels = original_labels.clone();
                self.cmr_labels(p);
                self.bandwidth();
                if self.bw < bw_0 {
                    bw_0 = self.bw;
                }
                iter_n = 0;
                nivel = 1;
                iter_k += 1;
            }
        }
        self.bw = bw_0;
    }

    #[inline(always)]
    fn perturbation(&mut self, nivel: usize, history: &mut HashMap<usize, HashSet<usize>>, options: &HashSet<usize>) {
        let mut iter = 0;
        let criticos = self.criticals();
        // Para todos os criticos
        for v in &criticos  {
            // let v = criticos.choose(&mut rand::thread_rng()).unwrap();
            while iter <= nivel {
                let u = v;
                if let Some(n_v) = history.get_mut(v) {
                    if n_v.len() >= self.labels.len() {
                        return ();
                    }
                    // Vec of vertices not in history of v
                    let n_v_copy = n_v.clone();
                    let diff: Vec<&usize> = options.symmetric_difference(&n_v_copy).collect();
                    let u = *diff.choose(&mut rand::thread_rng()).unwrap_or(&v);
                    n_v.insert(*u);
                    // print!("v= {};", v);
                    // print!("u={:?}; ", *u);
                    // print!("o={:?}; ", &options);
                    // println!("diff={:?}", diff);
                    // dbg!(&history);
                } else {
                    let u = self.labels.choose(&mut rand::thread_rng()).unwrap();
                    history.insert(*v, HashSet::from([*u]));
                    /* println!("\t primeiro");
                    dbg!(&h); */
                }
                let bw_0 = self.bw;
                self.labels.swap(*v, *u);
                if self.bandwidth_if_improves() <= bw_0 {
                    return ();
                }
                iter += 1;
            }
        }
    }

    // Proceeds with local search and change labels if a better labeling if found
    #[inline(always)]
    fn local_search(&mut self) {
        // TODO: Just find the first critical vertex/neighbours and optimize it
        let criticos = self.criticals();
        let mut bw_0 = self.bw;
        for v in criticos {
            for u in self.neighbour_of_criticals2(v) {
                if u == v {
                    continue;
                }
                self.labels.swap(v, u);
                if self.critical_improved(v) {
                    bw_0 = self.bw;
                } else {
                    // Return to previous situation
                    self.labels.swap(v, u);
                    // self.bw = bw_0;
                }
            }
        }
    }

    // Middle of rotulations of v
    #[inline(always)]
    fn mid(&self, neighbour: &mut &[usize]) -> usize {
        // mid(v) = ⌊(max(v) + min(v))/2⌋
        let mut neighbour: Vec<usize> = neighbour.to_vec();
        neighbour.sort_unstable();
        let min = neighbour.first().unwrap_or(&0);
        let max = neighbour.last().unwrap_or(&0);
        (min + max) / 2
        // (self.labels[*min] + self.labels[*max]) / 2
    }

    #[inline(always)]
    fn neighbour_of_criticals2(&self, v: usize) -> Vec<usize> {
        let neighbour = self.get_columns_of_row(self.old_label(v));

        let mut neighbour_of_criticals: Vec<usize> = Vec::new();
        // dbg!(v, neighbour, mid_v);
        for u in neighbour {
            let u = self.labels[*u];
            // let u = *u;
            if v != u {
                neighbour_of_criticals.push(u);
            }
        }

        let mid = self.mid(&mut neighbour_of_criticals.as_slice());

        neighbour_of_criticals.push(mid);
        neighbour_of_criticals.push(self.labels[mid]);

        neighbour_of_criticals
    }

    #[inline(always)]
    fn neighbour_of_criticals(&self, v: usize) -> Vec<usize> {
        // TODO: Ordenar em ordem crescente  do valor |mid(v) − f (u)|
        /*
        Trocar o rótulo de cada vértice crítico com os rótulos dos vértices u ∈ N ′ (v), em ordem crescente do valor |mid(v) − f (u)| até que se encontre uma solução melhor.
        O conjunto de candidatos a serem trocados com o vértice v é definido por
        N′(v) = {u : |mid(v) − f (u)| < |mid(v) − f (v)|} e
        mid(v) = ⌊(max(v) + min(v))/2⌋.
         */
        let mut neighbour_of_criticals: Vec<usize> = Vec::with_capacity(self.degree(v));
        let mut neighbour = self.get_columns_of_row(self.old_label(v));
        // let mid_v = self.mid(&mut neighbour);
        // dbg!(v, neighbour, mid_v);
        for u in neighbour {
            let u = self.labels[*u];
            if self.mid(&mut neighbour).abs_diff(u) <= self.mid(&mut neighbour).abs_diff(v) {
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
