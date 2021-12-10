use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct TorreHanoi {
    disco: [Vec<u32>; 3],     //Struct para a torre de hanoi
    tamanho: u32
}

impl TorreHanoi {
    pub fn new(n_disco: u32) -> Self { //implementação da torre de hanoi
        TorreHanoi {
            tamanho: n_disco,
            disco: [
                (1..=n_disco).rev().collect(),
                vec![],
                vec![]
            ]
        }
    }

    fn mover_disco(&mut self, torre1: usize, torre3: usize){ //mover o disco das torres
        let d = self.disco[torre1].pop().unwrap();
        self.disco[torre3].push(d);
        println!("{}", self);
    }

    fn resolver(&mut self, n: u32, torre1: usize, torre2: usize, torre3: usize){//Torre1:torre de origem,torre2:é a torre de destino torre3:toree de auxiliar
        if n>0 {
            self.resolver(n-1, torre1, torre3, torre2);//
            self.mover_disco(torre1, torre2);
            self.resolver(n-1, torre3, torre2, torre1);// a torre de origem a 3 a torre de destino vai ser a 2 e a torre auxiliar 1
        }
    }

    pub fn resultado(&mut self) {
        println!("{}", self);
        self.resolver(self.tamanho, 0, 1, 2);
    }
}

impl Display for TorreHanoi {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let s = self.tamanho as usize;
        for i in (0..s).rev() {
            for j in 0..(3*s) {
                if j%s==0 {write!(f, "║")?;}
                write!(f, "{}",
                    match self.disco[j/s].get(i) {
                        Some(&n) if (n as usize)>(j%s) => '─',
                        _                              => ' '
                    }
                )?;
            }
            write!(f, "║\n")?;
        }
        Ok(())
    }
}

fn main(){
    let n = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(4);
    TorreHanoi::new(n).resultado();
}
