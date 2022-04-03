mod ECC{
    use rug::{Integer, ops::PowAssign};
    
    fn temp(){
        
    }
    
    #[derive(Debug)]
    pub struct Point{
        pub x:Integer,
        pub y:Integer
    }

    #[derive(Debug)]
    pub struct EllipticCurve{
        spec_name:String,
        p:Integer,
        a:Integer,
        b:Integer,
        pub g:Point,
        n:Integer,
        h:Integer
    }

    impl EllipticCurve{



        /// Instantiate `Elliptic Curve` with secp256k1 
        pub fn new()->Self{
            EllipticCurve{
                spec_name: String::from("secp256k1"),
                p: Integer::from_str_radix("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",16).unwrap(),
                a: Integer::from(0),
                b: Integer::from(7),
                g: Point{ 
                    x: Integer::from_str_radix("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",16).unwrap(),
                    y: Integer::from_str_radix("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",16).unwrap()
                },
                n: Integer::from_str_radix("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",16).unwrap(),
                h: Integer::from(1)
            }
        }

        pub fn point_addition(self,p1:Option<Point>,p2:Option<Point>)->Option<Point>{
            assert_eq!(&self.is_on_curve(p1),true);
            assert_eq!(&self.is_on_curve(p2),true);

            if p1.is_none(){
                // 0 + p2 = p2
                return p2;
            }

            if p2.is_none(){
                // p1 + 0 = p1
                return p1;
            }

            let mut x1 = p1.unwrap().x;
            let mut y1 = p1.unwrap().y;

            let mut x2 = p2.unwrap().x;
            let mut y2 = p2.unwrap().y;

            if x1 == x2 && y1 != y2{
                // p + (-p) = 0
                return None
            }   



            return p1;

        }

        /// Generates public and private key pairs
        pub fn generate_pairs(){

        }

        /// Evaluates if the given `Point` lies on the defined `EllipticCurve`
        pub fn is_on_curve(self,point:Option<Point>)->bool{
            if point.is_none(){
                return true;
            }

            let eval_val = self.point_eval(&point.unwrap());
            return eval_val == Integer::from(0);
        }

        /// Evaluates `y^2 = x^3 + ax + b` at the given `Point` inside the filed `p`
        fn point_eval(self,point:&Point)->Integer{            
            let mut x_cube = Integer::from(&point.x);
            x_cube.pow_assign(3);

            let mut y_sq = Integer::from(&point.y);
            y_sq.pow_assign(2);

            let res = y_sq - x_cube - self.a * &point.x - self.b;
            return res % self.p;
        }
    }

     
}

#[cfg(test)]
mod test{
    use super::*;
    use rug::{Integer, ops::PowAssign};


    #[test]
    fn parameter_tests() {
        let ecc = ECC::EllipticCurve::new();
        println!("ECC struct is {:#?}",&ecc);
        
        let g = ECC::Point{ 
            x: Integer::from_str_radix("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",16).unwrap(),
            y: Integer::from_str_radix("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",16).unwrap()
        };
        


        assert_eq!(
            ecc.is_on_curve(Some(g)),
            true
        );
    }
}