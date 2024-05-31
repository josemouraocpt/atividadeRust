use std::io::{self};
use std::fs::{self, File};
use std::io::{stdin};
use std::path::Path;
use std::fs::read_to_string;
extern crate regex;
use regex::Regex;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Reserva{
    numero_reserva: i32,
    nome_hotel: String,
    numero_quarto: i32,
    data_check_in: String,
    data_check_out: String,
    hash_check_in: String
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Hash{
    key: String,
    reserva: Reserva
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct HashMap{
    values: Vec<Hash>
}


impl Reserva{
    fn new(numero_reserva: i32, nome_hotel: String, numero_quarto: i32, data_check_in: String, data_check_out: String) -> Reserva{
        let hash_check_in = Self::hash_check_in(data_check_in.clone());
        Reserva{
            numero_reserva,
            nome_hotel,
            numero_quarto,
            data_check_in,
            data_check_out,
            hash_check_in
        }
    }
    fn hash_check_in(data_check_in: String) -> String{
        let dia = &data_check_in[..2];
        let mes = &data_check_in[3..5];
        let ano = &data_check_in[6..];
        let hashed_date = mes.to_owned() + &((&dia.parse::<i32>().unwrap()* &mes.parse::<i32>().unwrap() * &ano.parse::<i32>().unwrap()) % 3).to_string() + ano + dia;
        return  hashed_date;
    }
}

impl HashMap{
    fn new() -> HashMap{
        HashMap{
            values: Vec::new()
        }
    }

    fn push(&mut self, hash: Hash){
        //Antes de inserir um novo valor, é verificado se já existe um hash na HASHMAP, caso exista o valor antigo é removido e o novo inserido
        let index: usize = hash.key[..2].parse().unwrap();
        if self.values.get(index) == None{
            self.values.push(hash);
        }else{
            self.remove(hash.key.clone());
            self.values.push(hash);
        }
        return;
    }

    fn display(&self){
        for (item, index) in self.values.iter().enumerate(){
            println!("OPÇÃO - {}\nHASH: {}\nRESERVA: {}\nHOTEL: {}\n", item, index.key, index.reserva.numero_reserva, index.reserva.nome_hotel);
        }
        return;
    }

    fn get(&self, target: usize){
        //Binary search para encontrar o valor da reserva
        let mut low = 0;
        let mut high = self.values.len() - 1;
        //let index: usize = target[..2].parse().unwrap();
        while low <= high {
            let mid = low + (high - low) / 2;
            if self.values[mid] == self.values[target]{
                //Se possivel incluir cores nos prints
                println!("Detalhes da Reserva {}:",target);
                println!("Chave: {}", self.values[mid].key);
                println!("Número da reserva: {}", self.values[mid].reserva.numero_reserva);
                println!("Nome do hotel: {}",self.values[mid].reserva.nome_hotel);
                println!("Número do quarto: {}",self.values[mid].reserva.numero_quarto);
                println!("Data do check-in: {}",self.values[mid].reserva.data_check_in);
                println!("Data do checK-out: {}\n",self.values[mid].reserva.data_check_out);
                return;
            } else if self.values[mid] < self.values[target]{
                low = mid + 1
            } else{
                high = mid - 1
            }
        }
        println!("Reserva não encontrada!\n");
        return;
    }

    fn remove(&mut self, target: String){
        //tratar o valor caso seja menor que 10
        let index: usize = target[..2].parse().unwrap();
        println!("Reserva removida!\n");
        self.values.remove(index);
        return;
    }

    fn show_struct(&self){
        println!("Estrutura do HASPMAP:\n{:#?}", self.values);
    }

}


impl Hash{
    fn new(reserva: Reserva, size: i32) -> Hash{
        let nome_copy = &reserva.nome_hotel;
        let data_copy = &reserva.data_check_in;
        let key = Self::simple_hash(size, reserva.numero_reserva, reserva.numero_quarto,nome_copy.to_string(), data_copy.to_string());
        Hash{
            key,
            reserva
        }
    }
    fn simple_hash(size: i32,numero_reserva: i32, numero_quarto: i32, nome_hotel: String, data_check_in: String) -> String{   
        let hash_value = (((numero_reserva / nome_hotel.len() as i32) * numero_quarto) % 7) as i64;
        let mut hash: String = String::new();
        //Tratando o tamando do caracter inicial da hash, os valores possivel são 01 e 99.
        if size < 10{
            hash = "0".to_owned() + &size.to_string() + &data_check_in[..2].to_owned() + &hash_value.to_string() +  &nome_hotel.replace(" ", "")[..3].to_uppercase() +  &data_check_in[6..];
        } else{
            hash = size.to_string() + &data_check_in[..2].to_owned() + &hash_value.to_string() +  &nome_hotel.replace(" ", "")[..3].to_uppercase() +  &data_check_in[6..];
        }
        return hash;
    }
}

fn main() {
    let path = Path::new("./reservas.txt");
    let mut lines = Vec::new();
    let binding = read_to_string(path).unwrap();
    for line in binding.lines(){
        let line_arr: Vec<&str> = line.trim().split(&[',', ':'][..]).collect();
        lines.push(line_arr);
    }
    let hash_postions: i32 = lines[0][0].parse().unwrap();
    lines.remove(0);
    let mut hash_map: HashMap = HashMap::new();
    for line in lines.iter(){
        let reserva: Reserva = Reserva::new(line[0].trim_start().parse::<i32>().unwrap(), line[1].trim_start().to_owned(), line[2].trim_start().parse::<i32>().unwrap(), line[3].trim_start().to_owned(), line[4].trim_start().to_owned());
        let hash: Hash = Hash::new(reserva, hash_map.values.len() as i32);
        hash_map.push(hash);
    }
    println!("Bem-vindo ao sistema de reserva de hotéis.\n");
    loop{
        let mut option: String = String::new();
        let mut num_reserva: String = String::new();
        let mut nm_re: String = String::new();
        let mut nm_ht: String = String::new();
        let mut nm_qt: String = String::new();
        let mut data_in: String = String::new();
        let mut data_ou: String = String::new();
        let re_num = Regex::new(r"[a-zA-Z]|\d{5}").unwrap();
        let re_date = Regex::new(r"\b(0[1-9]|[12][0-9]|3[01])/(0[1-9]|1[0-2])/\d{4}\b").unwrap();
        let mut can_continue = false;

        print!("Para ver informações sobre as reservas pressione insira: RESERVAS\n");
        print!("Para incluir uma nova reserva insira: NOVA\n");
        print!("Para cancelar uma reserva insira: CANCELAR\n");
        print!("Para remover uma reserva insira: REMOVER\n");
        print!("Para realizar o check-out de uma reserva insira: CHECK-OUT\n");
        print!("Para ver a estrutura do HASHMAP insira: ESTRUTURA\n");
        print!("Para sair da aplicação insira: SAIR\n");
        io::stdin().read_line(&mut option).unwrap();

        match option.trim(){
            "SAIR" | "sair" | "Sair" => break,
            "RESERVAS" | "reservas" | "Reservas" => {
                println!("Qual reserva você deseja ver mais informações:\n");
                hash_map.display();
                println!("\nEscolha entre as opções 0 a {}: ", hash_map.values.len()-1);
                io::stdin().read_line(&mut num_reserva).expect("Insira um valor númerico válido");

                can_continue = !re_num.is_match(&num_reserva);
                while can_continue == false {
                    num_reserva = "".to_string();
                    println!("\nEscolha entre as opções 0 a {}: ", hash_map.values.len()-1);
                    io::stdin().read_line(&mut num_reserva).expect("Insira um valor númerico válido");
                    can_continue = !re_num.is_match(&num_reserva);
                }

                let res: i32 = num_reserva.trim().parse().unwrap();
                if res > (hash_map.values.len()-1) as i32 || res < 0{
                    println!("Valor inserido não é válido!");
                    continue;
                } else{
                    let target: usize = num_reserva.trim().parse().unwrap();
                    hash_map.get(target);
                }
            },
            "CANCELAR" | "REMOVER"| "CHECK-OUT" | "cancelar" | "Cancelar" | "remover" | "Remover" | "check-out" | "Check-out" => {
                println!("Qual reserva você deseja {}:\n", option.to_lowercase());
                hash_map.display();
                println!("\nEscolha entre as opções 0 a {}: ", hash_map.values.len()-1);
                io::stdin().read_line(&mut num_reserva).expect("Insira um valor númerico válido");

                can_continue = !re_num.is_match(&num_reserva);
                while can_continue == false {
                    num_reserva = "".to_string();
                    println!("\nEscolha entre as opções 0 a {}: ", hash_map.values.len()-1);
                    io::stdin().read_line(&mut num_reserva).expect("Insira um valor númerico válido");
                    can_continue = !re_num.is_match(&num_reserva);
                }

                let res: i32 = num_reserva.trim().parse().unwrap();
                if res > (hash_map.values.len()-1) as i32 || res < 0{
                    println!("Valor inserido não é válido!");
                    continue;
                } else{
                    if res < 10{
                        let res_str = "0".to_owned() + &res.to_string();
                        hash_map.remove(res_str);
                    }else {
                        hash_map.remove(res.to_string());
                    }
                }
            },
            "NOVA" | "nova" | "Nova" => {
                print!("Insira as informações a seguir para inserir uma nova reserva:\n");

                println!("Insira o número da reserva: APENAS NÚMEROS ENTRE 0 E 1000");
                io::stdin().read_line(&mut nm_re).expect("Insira um valor númerico válido");

                can_continue = !re_num.is_match(&nm_re);
                while can_continue == false{
                    nm_re = "".to_string();
                    println!("Insira o número da reserva: APENAS NÚMEROS ENTRE 0 E 1000");
                    io::stdin().read_line(&mut nm_re).expect("Insira um valor númerico válido");
                    can_continue = !re_num.is_match(&nm_re);
                }
                
                println!("Insira o nome do hotel: ");
                stdin().read_line(&mut nm_ht).unwrap();
                while nm_ht.len() == 2{
                    println!("Insira o nome do hotel: ");
                    stdin().read_line(&mut nm_ht).unwrap();
                }
                
                println!("Insira o número do quarto: APENAS NÚMEROS ENTRE 0 E 1000");
                io::stdin().read_line(&mut nm_qt).expect("Insira um valor númerico válido");

                can_continue = !re_num.is_match(&nm_qt);
                while can_continue == false {
                    nm_qt = "".to_string();
                    println!("Insira o número do quarto: APENAS NÚMEROS ENTRE 0 E 1000");
                    io::stdin().read_line(&mut nm_qt).expect("Insira um valor númerico válido");
                    can_continue = !re_num.is_match(&nm_qt);
                }
            
                println!("Insira a data do check-in: Padrão 01/01/2000");
                stdin().read_line(&mut data_in).expect("Insira uma data válida");

                can_continue = re_date.is_match(&data_in);
                while can_continue == false {
                    data_in = "".to_string();
                    println!("Insira a data do check-in: Padrão 01/01/2000");
                    stdin().read_line(&mut data_in).expect("Insira uma data válida");
                    can_continue = re_date.is_match(&data_in);
                }
                
                println!("Insira a data do check-out: Padrão 01/01/2000");
                stdin().read_line(&mut data_ou).expect("Insira uma data válida");

                can_continue = re_date.is_match(&data_ou);
                while can_continue == false {
                    data_ou = "".to_string();
                    println!("Insira a data do check-out: Padrão 01/01/2000");
                    stdin().read_line(&mut data_ou).expect("Insira uma data válida");
                    can_continue = re_date.is_match(&data_ou);
                }
                
                
                let num_res: i32 = nm_re.trim().parse().unwrap();
                let nom_hot: String = nm_ht.trim().parse().unwrap();
                let num_qua: i32 = nm_qt.trim().parse().unwrap();
                let dt_in: String = data_in.trim().parse().unwrap();
                let dt_out: String = data_ou.trim().parse().unwrap();
                let reserva: Reserva = Reserva::new(num_res, nom_hot, num_qua, dt_in, dt_out);
                let hash: Hash = Hash::new(reserva, hash_map.values.len() as i32);
                hash_map.push(hash);

            },
            "ESTRUTURA" | "estrutura" | "Estrutura" => {
                hash_map.show_struct();
            }
            _ => println!("Entrada não aceita!\nInsira um novo valor!")
        }
    }
    println!("Obrigado por visitar!");
}