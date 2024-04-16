#[derive(Clone, Copy, Default)]
pub struct CollisionFilter {
    category: u16,
    mask: u32,
    group: u16,
}

impl CollisionFilter {
    pub fn new(category: u16, mask: u32, group: u16) -> Self {
        CollisionFilter {
            category: category,
            mask: mask,
            group: group,
        }
    }

    pub fn get_category(&self) -> u16 {
        self.category
    }

    pub fn get_mask(&self) -> u32 {
        self.mask
    }

    pub fn get_group(&self) -> u16 {
        self.group
    }
}
