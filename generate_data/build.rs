fn main() {
  // recompile when data changes
  println!("cargo:rerun-if-changed=src/data/");
}
