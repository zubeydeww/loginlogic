fn main() {



    let mut database: Vec<Kullanici> = Vec::new();


    register("sad".to_string(), 12, "hayat@gmail.com".to_string(), "şifre".to_string(),&mut database);
    login("hayat@gmail.com".to_string(), "şif".to_string(), &mut database);

    

   
}

#[derive(Debug,Clone)]
struct Kullanici{
    isim: String,
    yas: u32,
    email: String,
    şifre: String
}


fn login(email:String,şifre:String,database:&mut Vec<Kullanici>) {
    for data in database {
        if email == data.email && şifre==data.şifre {

            println!("Giriş başarılı")
            
        }else {
            println!("Giriş başarısız")
        }
        
    }
    
}


fn register(isim:String,yas:u32,email:String,şifre:String,database:&mut Vec<Kullanici>) {

    let kullanıcı = Kullanici{
        isim:isim,
        yas:yas,
        email:email,
        şifre:şifre
    };


    database.push(kullanıcı);

    println!("Kullanıcı oluşturuldu {:?}",database)
    
}





