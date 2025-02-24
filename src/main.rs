use shamir_secret_sharing::shamir::Shamir;
fn main() {
   let shamir = Shamir::new(3, 5, 97);
   let secret = 42;

   let shares = shamir.split_secret(secret);
   println!("Generates shares are : {:?}",shares);

   let reconstructed = shamir.reconstruct_secret(&shares[..3]);
   println!("Reconstructed Secret: {}", reconstructed);

   assert_eq!(secret,reconstructed);

}
