fn main() {
    fn get_all_combos(list: Vec<&str>) -> Vec<String> {
        fn gen_combo(prefix: String, chars: Vec<&str>, combos: &mut Vec<String>) {
            for (i, el) in chars.clone().iter().enumerate() {
                let new_prefix = prefix.to_owned() + &el.to_owned();
                combos.push(new_prefix.clone());
                gen_combo(new_prefix, chars[i + 1..].to_vec(), combos);
            }
        }
        let mut combos: Vec<String> = vec![];
        gen_combo("".to_owned(), list, &mut combos);
        combos
    }
    let chars = vec!["a", "b", "c", "d"];
    println!("all combos: {:?}", get_all_combos(chars));
}
