const MAX_SEGMENTS: usize = 20;

enum ProgressType {
    /// The upper limit is known
    Limited(LimitedType),

    /// The upper limit is unknown
    Unlimited(UnlimitedType),
}
enum LimitedInfoDisplay {
    Percent,
    Cases,
    None,
}
struct LimitedType {
    upper: usize,
    display_type: LimitedInfoDisplay,
}
impl LimitedType {
    fn new(upper: usize) -> Self {
        Self {
            upper,
            display_type: LimitedInfoDisplay::Percent,
        }
    }
}

struct UnlimitedType {
    animation_pos: usize,
}
impl UnlimitedType {
    fn new() -> Self {
        Self { animation_pos: 0 }
    }
}

struct Progress<I: Iterator> {
    iter: I,
    current: usize,
    r#type: ProgressType,
}
impl<I: Iterator> Iterator for Progress<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 0 {
            print!("\x1B[1A"); // Up one line
            print!("\x1B[2K"); // Clear current line
        }
        match &mut self.r#type {
            ProgressType::Limited(cfg) => {
                let percent = self.current as f64 / cfg.upper as f64;
                let filled_segments = percent * (MAX_SEGMENTS as f64);
                print!("[{}] ", " ".repeat(MAX_SEGMENTS));
                match cfg.display_type {
                    LimitedInfoDisplay::Percent => println!("{}%", (percent * 100.).round()),
                    LimitedInfoDisplay::Cases => println!("{}/{}", self.current, cfg.upper),
                    _ => println!(""),
                }
                print!("\x1B[1A"); // Up one line
                println!("[{}", "=".repeat(filled_segments as usize));
            }
            ProgressType::Unlimited(ref mut cfg) => {
                println!("[{}]", " ".repeat(MAX_SEGMENTS));
                print!("\x1B[1A");
                println!("[{}=", " ".repeat(cfg.animation_pos));
                cfg.animation_pos += (cfg.animation_pos + 1) % MAX_SEGMENTS;
            }
            _ => {
                unimplemented!();
            }
        }
        self.current += 1;
        self.iter.next()
    }
}
trait ProgressDisplay {
    fn progress(self) -> Progress<Self>
    where
        Self: Sized + Iterator,
    {
        let (_, upper) = self.size_hint();
        let ptype = upper.map_or(
            ProgressType::Unlimited(UnlimitedType::new()),
            |upper| ProgressType::Limited(LimitedType::new(upper)),
        );
        Progress {
            iter: self,
            current: 0,
            r#type: ptype,
        }
    }
}
impl<I: Iterator> ProgressDisplay for I {}

#[cfg(test)]
mod tests {
    #[test]
    fn limited() {
        use crate::ProgressDisplay;
        let range = (0..100).into_iter();
        for _ in range.progress() {
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    #[test]
    fn unlimited() {
        use crate::ProgressDisplay;
        let range = (0..).into_iter();
        for i in range.progress() {
            std::thread::sleep(std::time::Duration::from_millis(50));
            if i == 100 {
                break;
            }
        }
    }
}
