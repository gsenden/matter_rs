use uuid::Uuid;

pub const BASE_DELTA: f64 = 1000. / 60.;

#[derive(Clone, Copy, Default)]
pub enum ShapeType {
    #[default]
    Body,
}

pub struct OrderedHashMap<T> {
    pub keys: Vec<String>,
    pub values: Vec<T>,
}

impl<T> OrderedHashMap<T> {
    pub fn new() -> OrderedHashMap<T> {
        OrderedHashMap {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: T) {
        let mut index = 0;
        let mut found = false;
        while index < self.keys.len() {
            if self.keys[index] == key {
                found = true;
                break;
            }
            index += 1
        }
        if found {
            self.values[index] = value;
        } else {
            self.keys.push(key);
            self.values.push(value);
        }
    }
}

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn next_id() -> uuid::Uuid {
    Uuid::new_v4()
}

pub fn sign(value: f64) -> i8 {
    if value < 0. {
        -1
    } else {
        1
    }
}
