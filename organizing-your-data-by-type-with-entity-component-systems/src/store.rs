use crate::gen::GenerationData;

// This could be implemented by Vec type object
// or tree or hashmap depending on how full
// you expect it to be
pub trait EcsStore<T> {
    fn add(&mut self, gen_data: GenerationData, data: T);
    fn get(&self, gen_data: GenerationData) -> Option<&T>;
    fn get_mut(&mut self, gen_data: GenerationData) -> Option<&mut T>;
    fn drop(&mut self, gen_data: GenerationData);

    // Optional, could be impl by iter trait
    fn for_each<F: FnMut(GenerationData, &T)>(&self, f: F);
    fn for_each_mut<F: FnMut(GenerationData, &mut T)>(&mut self, f: F);
}

pub struct VecStore<T> {
    items: Vec<Option<(u64, T)>>,
}

impl<T> VecStore<T> {
    pub fn new() -> Self {
        VecStore { items: Vec::new() }
    }
}

impl<T> EcsStore<T> for VecStore<T> {
    fn add(&mut self, gen_data: GenerationData, data: T) {
        while gen_data.position >= self.items.len() {
            self.items.push(None);
        }
        self.items[gen_data.position] = Some((gen_data.generation, data));
    }

    fn get(&self, gen_data: GenerationData) -> Option<&T> {
        // get returns option, vec holds options
        if let Some(Some((inner_gen, data))) = self.items.get(gen_data.position) {
            if *inner_gen == gen_data.generation {
                return Some(data);
            }
        }

        None
    }

    fn get_mut(&mut self, gen_data: GenerationData) -> Option<&mut T> {
        if let Some(Some((inner_gen, data))) = self.items.get_mut(gen_data.position) {
            if *inner_gen == gen_data.generation {
                return Some(data);
            }
        }

        None
    }

    fn drop(&mut self, gen_data: GenerationData) {
        if let Some(Some((inner_gen, _))) = self.items.get(gen_data.position) {
            if *inner_gen == gen_data.generation {
                self.items[gen_data.position] = None;
            }
        }
    }

    fn for_each<F: FnMut(GenerationData, &T)>(&self, mut f: F) {
        for (num, idx) in self.items.iter().enumerate() {
            if let Some((inner_gen, data)) = idx {
                f(
                    GenerationData {
                        generation: *inner_gen,
                        position: num,
                    },
                    data,
                );
            }
        }
    }

    fn for_each_mut<F: FnMut(GenerationData, &mut T)>(&mut self, mut f: F) {
        for (num, idx) in self.items.iter_mut().enumerate() {
            if let Some((inner_gen, data)) = idx {
                f(
                    GenerationData {
                        generation: *inner_gen,
                        position: num,
                    },
                    data,
                );
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::gen::GenerationManager;

    #[test]
    fn test_store_can_drop() {
        let mut gm = GenerationManager::new();
        let mut vs = VecStore::new();

        vs.add(gm.next(), 5);
        vs.add(gm.next(), 3);
        vs.add(gm.next(), 2);

        let g4 = gm.next();
        vs.add(g4, 5);

        vs.for_each_mut(|g, d| *d += 2);

        assert_eq!(vs.get(g4), Some(&7));

        vs.drop(g4);

        assert_eq!(vs.get(g4), None);
    }
}
