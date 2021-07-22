fn song() {
  for i in 0..12 {
    println!(
      "On the {} day of Christmas my true love gave to me",
      INDICES[i]
    )
  }
}

const INDICES: [&str; 12] = [
  "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
  "eleventh", "twelfth",
];
