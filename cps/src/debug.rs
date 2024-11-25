pub trait Debuggable {
    fn debug_string(&self) -> String;
}

impl<T: std::fmt::Debug + std::fmt::Display> Debuggable for Vec<T> {
    fn debug_string(&self) -> String {
        use itertools::Itertools;
        use std::iter::repeat;
        if let Some(max_size) = self.iter()
            .map(|x| format!("{:?}", x).len())
            .max() {
                let mut idx = String::from("idx |");   
                let mut val = String::from("val |");   
                for (i, xi) in self.iter().enumerate() {
                    idx.push_str(
                        &format!(" {:>w$} |", i, w=max_size)
                    );
                    val.push_str(
                        &format!(" {:>w$} |", xi, w=max_size)
                    );
                }

                format!("{}\n{}\n{}\n", idx, repeat('-').take(idx.len()).join(""), val)
            } else {
                format!("idx | \nval |\n")
            }
    }
}

#[macro_export]
macro_rules! dbg {
    ($val:expr) => {
        #[cfg(feature="local")]
        {
            use Debuggable;
            let val = &$val;
            eprintln!("{}", val.debug_string());
            val
        }
    };
}
