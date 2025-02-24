use rand::Rng;

pub struct Shamir {
    threshold : usize,
    shares : usize,
    prime : u64,
}

impl Shamir{

    pub fn new(threshold : usize ,shares : usize , prime : u64)->Self{
        assert!(threshold<=shares , "Threshold cannot be greater than the shares");
        Self { threshold, shares, prime }
    }


    pub fn split_secret(&self , secret : u64) -> Vec<(u64,u64)>{
        let mut rng = rand::thread_rng();
        let mut coeffs = vec![secret];
        for _ in 1..self.threshold{
            coeffs.push(rng.gen_range(1..self.prime));
        }

        let mut shares = Vec::new();

        for x in 1..=self.shares as u64{
            let mut y = 0;
            for(i,&coeff) in coeffs.iter().enumerate(){
                y+=coeff*x.pow(i as u32) % self.prime;
            }
            y%=self.prime;
            shares.push((x,y));
        }
        shares
    }

    pub fn reconstruct_secret(&self, shares: &[(u64,u64)])->u64{
        assert!(shares.len>=self.threshold ,  "Not enough shares to reconstruct the secret");

        let mut secret = 0;

        for (i,&(xi,yi)) in shares.iter().enumerate(){
            let mut num =1;
            let mut denom =1;

            for (j,&(xj,_)) in shares.iter().enumerate(){
                if i!=j{
                    num = num*(0-xj)%self.prime;
                    denom = denom*(xi-xj)%self.prime;
                }
            }

            let lagrange_coeff = (num* mod_inverse(denom,self.prime))%self.prime;
            secret = (secret + yi*lagrange_coeff)%self.prime;

        }
        secret
    }

}
fn mod_inverse(a: u64, prime: u64) -> u64 {
    let mut mn = (prime, a);
    let mut xy = (0, 1);
    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }
    ((xy.0 + prime) % prime) as u64
}
