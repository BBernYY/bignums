struct Number {
    data: Vec<u8>,
    sign: bool,
}
impl Number {
    fn new(data: Vec<u8>, sign: bool) -> Self {
        Number {
            data: data,
            sign: sign
        }
    }
    fn clone(&self) -> Number {
        Number {
            data: self.data.clone(),
            sign: self.sign.clone()
        }
    }
    fn divpow(&self, pow: usize) -> (Number, Vec<u8>) {
        (Self::new(self.data[pow..].to_vec(), self.sign.clone()), self.data[0..pow].to_vec())
    }
    fn repow(&self, pow: usize, missed: Option<Vec<u8>>) -> Number {
        let mut o = self.clone();
        o.data.splice(0..0,missed.unwrap_or_else(|| vec![0; pow]));
        o
    }
    fn add_u8(&self, num: u8) -> Number {
        let first = self.clone().stripped().data[0];
        let (sum, ovf) = first.overflowing_add(num);
        let mut new_vec = Vec::with_capacity(self.data.len());
        new_vec.push(sum);             // Push the new first element
        new_vec.extend_from_slice(&self.data[1..]); // Append the rest
        let mut out: Number = Self::new(new_vec, false);
        if ovf {
            if out.data.len() > 1 {
                out = out.divpow(1).0.add_u8(1);
                out.data.insert(0,sum);
            } else {
                out.data.push(1)
            }
        }
        out
}
    fn stripped(&self) -> Number {
        let mut out = self.clone();
        for i in 0..self.data.len() {
            if self.data[self.data.len()-1-i] != 0 {
                break
            }
            out.data.pop();
        }
        if out.data.len() == 0 {
            out = Self::new(vec![0], false);
        }
        out
    }
    fn add(&self, num2: Number) -> Number {
        let mut num1 = self.clone();
        for n in 0..num2.data.len() {
            let c = num2.data[n];
            let extra: Vec<u8>;
            (num1, extra) = num1.divpow(n);
            num1 = num1.stripped().add_u8(c).repow(n, Some(extra));
        }
        num1
    }
    fn subtract(&self, num: Number) -> Number{
        let mut num2 = num.clone().stripped();
        let mut num1 = self.clone().stripped();
        let diff: i8 = num2.data.len() as i8 - num1.data.len() as i8;
        if diff > 0 {
            num1.data.extend(vec![0; diff as usize]);
        } else if diff < 0 {
            num2.data.extend(vec![0; -diff as usize]);
        }
        for i in 0..num2.data.len()
        {
            let (result, overflowed) = num1.data[i].overflowing_sub(num2.data[i]);
            if overflowed {
                if i == num1.data.len()-1 {
                    num1.sign = !num1.sign;
                } else {
                    num1 = num1.subtract(Self::new(vec![1], false).repow(i+1, None));
                }
            }
            num1.data[i] = result;
        }
        num1
    }
    fn greaterthan(&self, num2: Number) -> bool {
        let mut num1 = self.clone().stripped();
        let diff: i8 = num2.data.len() as i8 - num1.data.len() as i8;
        if diff > 0 {
            return false;
        } else if diff < 0 {
            return true;
        }
        num1.data.last() > num2.data.last()


    }
    fn multiply(&self, num2: Number) -> Number {
        let mut num1 = Self::new(self.data.clone(), false);
        let (len1, len2) = (num1.data.len(), num2.data.len());
        let mut out = Number {
            data: vec![0u8; len1+len2+1],
            sign: false
        };
        for i in 0..len1 {
            for j in 0..len2 {
                let value = (num1.data[i] as u16)*(num2.data[j] as u16);
                let n = Self::new(vec![(value & 0xFF) as u8, (value >> 8) as u8], false).repow(i+j, None);
                out = out.add(n);
            }
        }
        out
    }

    fn from_str(txt: &str) -> Self {
        let mut out = Self::new(vec![0], false);
        let mut c = txt.clone();
        if c.starts_with('-') {
            c = &c[1..]
        }
        for i in c.chars() {
            //println!("{:?}", out.data);
            let n = i as u8 - b'0';
            out = out.multiply(Self::new(vec![10], false));
            out = out.add_u8(n);
        }
        out.stripped()
    }
    fn intdiv(&self, num: Number) -> (Number, Number){
        let mut num1 = self.clone();
        let mut c = Self::new(vec![0], false);
        let num2 = num.clone();
        loop {
            if num2.greaterthan(num1.clone()) {
                break;
            }
            num1 = num1.subtract(num2.clone());
            c = c.add_u8(1);
        }

        (c, num1)

    }
    fn longdiv(&self, num: Number) -> (Number, Number){
        let mut num1 = self.clone();

        let mut num2 = num.clone();
        let mut div;
        let mut d = Self::new(vec![0], false);
        let mut num1new;
        loop {
            let mut seg: usize = num1.clone().data.len()-num2.data.len();
            if seg > 0 {seg -= 1;};
            let (dp, rest) = num1.divpow(seg);
            (div, num1new) = dp.intdiv(num2.clone());
            d = d.add(div.repow(seg, None).stripped());
            num1 = num1new.repow(seg, Some(rest)).stripped().clone();
            if num2.greaterthan(num1.clone()) {
                break
            }
            
        }
        (d, num1)
    }
    fn repr(&self) -> String{
        let mut num = self.clone();
        let mut out = String::new();
        let mut rem: Number;
        loop { 
            (num, rem) = num.longdiv(Self::new(vec![10], false));
            out.push((rem.data[0] + b'0') as char);
            if num.subtract(Self::new(vec![10], false)).sign {
                out.push((num.data[0] + b'0') as char);
                break
            }
        }
        out.chars().rev().collect()
    }
    

}
fn main() {
    //let mut i: u32 = 0;
    //let mut n = Number::new(0);
    //for _ in 0..10000 {
        //n = n.add_u8(100);
        //i += 100;
        //println!("{:?}", n.data);
    //}
    //println!("{}", i);
    //let mut n = Number::new(vec![25, 1, 0]);
    //n = n.multiply(Number::new(vec![4,4]));
    //println!("{}", (25+256)*(4+4*256));
    let mut n = Number::from_str("686458934738496234837254823589437549385638946534685934759347578934573497853475934759347593475927345784");
    n = n.multiply(Number::from_str("345896435843765834765874309583409583420985293847592803467582347856283467582738946529387457892346985679858975696"));
    //let mut n = Number::from_str("110010");
    //println!("{:?}", n.data);
    let (a, b): (Number, Number) = n.longdiv(Number::from_str("134847534957"));
    println!("{}, {}", a.repr(), b.repr());
   // println!("{:?} {:?}", a.0.data, a.1.data);

    //println!("{}", (n.data[0] as u32)+(n.data[1] as u32)*256+(n.data[2] as u32)*256*256);
}
