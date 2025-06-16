struct Number {
    data: Vec<u8>,
}
impl Number {
    fn new(data: Vec<u8>) -> Self {
        Number {
            data: data,
        }
    }
    fn divpow(&self, pow: usize) -> (Number, Vec<u8>) {
        (Self::new(self.data[pow..].to_vec()), self.data[0..pow].to_vec())
    }
    fn repow(&self, pow: usize, missed: Option<Vec<u8>>) -> Number {
        let mut o = self.data.clone();
        o.splice(0..0,missed.unwrap_or_else(|| vec![0; pow]));
        Self::new(o)
    }
    fn add_u8(&self, num: u8) -> Number {
        let first = self.data[0];
        let (sum, ovf) = first.overflowing_add(num);
        let mut new_vec = Vec::with_capacity(self.data.len());
        new_vec.push(sum);             // Push the new first element
        new_vec.extend_from_slice(&self.data[1..]); // Append the rest
        let mut out: Number = Self::new(new_vec);
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
    fn add(&self, num2: Number) -> Number {
        let mut num1 = Self::new(self.data.clone());
        for n in 0..num2.data.len() {
            let c = num2.data[n];
            let extra: Vec<u8>;
            (num1, extra) = num1.divpow(n);
            num1 = num1.add_u8(c).repow(n, Some(extra));
        }
        num1
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
    //println!("{}", (n.data[0] as u32)+(n.data[1] as u32)*256+(n.data[2] as u32)*256*256);
    let mut n = Number::new(vec![65, 110]);
    println!("{:?}", n.data);
    n = n.add(Number::new(vec![23, 238, 103]));
    println!("{:?}", n.data);
}
