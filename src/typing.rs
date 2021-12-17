use std::fmt;

pub union Val {
    i: i32,
    f: f32,
}

pub struct Num {
    typ: char, // Hopefully can get away with just i, f, etc. Will swap to String if one char is insufficient
    val: Val,
}

impl fmt::Debug for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *&self.typ == 'i'{
            unsafe{ f.write_fmt(format_args!("{}", &self.val.i))}
        } else if *&self.typ == 'f'{
            unsafe{ f.write_fmt(format_args!("{}", &self.val.f))}
        } else {
            f.write_fmt(format_args!("{}?", &self.typ)) //weird type wtf?
        }
    }
}
