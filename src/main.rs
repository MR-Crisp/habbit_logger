use std::fs;

fn main(){
    write_file()
}

fn read_file()-> String {
    let data = fs::read_to_string( "./info.csv").expect("Unable to read file");
    return data;
}

fn write_file(){
    let data = read_file();
    let parts: Vec<&str> = data.split("|").collect();
    let parts_iter = parts.iter();
    for value in parts_iter{
        print!("{}\n",value)

    }

}