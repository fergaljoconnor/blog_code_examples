use vectorize::vectorize;

#[vectorize(Potatoes)]
#[derive(Debug)]
struct Potato {
    starch_content: f32,
}

#[cfg(test)]
mod tests {
    use super::{Potato, Potatoes};
    #[test]
    fn it_works() {
        let mut potatoes = Potatoes::new();

        for starch_content in &[0.5, 0.1, 0.8] {
            potatoes.push(Potato {starch_content: *starch_content});
        }

        let total_starch: f32 = potatoes.get_starch_content().iter().sum();
        assert!((total_starch - 1.4).abs() < 0.001);
    }
}
