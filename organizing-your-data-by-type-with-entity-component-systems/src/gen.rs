#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GenerationData {
    pub position: usize,
    pub generation: u64,
}

pub struct EntityActive {
    active: bool,
    generation: u64,
}

// where we get new GenerationIDs from
pub struct GenerationManager {
    items: Vec<EntityActive>,
    // list of all dropped entities
    drops: Vec<usize>,
}

impl GenerationManager {
    pub fn new() -> Self {
        GenerationManager {
            items: Vec::new(),
            drops: Vec::new(),
        }
    }

    pub fn next(&mut self) -> GenerationData {
        if let Some(location) = self.drops.pop() {
            // most recent drop
            let ea = &mut self.items[location];
            ea.active = true;
            ea.generation += 1;
            return GenerationData {
                position: location,
                generation: ea.generation,
            };
        }
        //if noting in the drops vec, add on the end
        self.items.push(EntityActive {
            active: true,
            generation: 0,
        });
        return GenerationData {
            position: self.items.len() - 1,
            generation: 0,
        };
    }

    pub fn drop(&mut self, gen_data: GenerationData) {
        if let Some(ea) = self.items.get_mut(gen_data.position) {
            if ea.active && ea.generation == gen_data.generation {
                // do not drop newer items than the given
                ea.active = false;
                self.drops.push(gen_data.position);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_items_drop() {
        let mut gm = GenerationManager::new();

        let g = gm.next();
        assert_eq!(
            g,
            GenerationData {
                generation: 0,
                position: 0,
            }
        );
        let g2 = gm.next();
        gm.next();
        gm.next();
        gm.drop(g2);
        let g3 = gm.next();

        assert_eq!(
            g3,
            GenerationData {
                generation: 1,
                position: 1,
            }
        );
    }
}
