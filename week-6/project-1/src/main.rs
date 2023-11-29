use std::io;

fn main() {
    let mut name = String::new();
    let mut dob = String::new();
    let mut email = String::new();
    let mut phone_no = String::new();
    let mut siblings_no = String::new();
    let mut diagnosis = String::new();
    let mut children_no = String::new();
    let mut village_name = String::new();
    let mut age = String::new();

    let a = "Alzheimer";
    let b = "Arrhythmia";
    let c = "Chronic Kidney Disease";
    let d = "Diabetes";
    let e = "Arthritis";

    loop{
        println!("HEALTH DIAGNOSIS           AMOUNT       VILLAGE   DISCOUNT
                 a. Alzheimer               1,200,000     Akpabom     10%\n
                 b. Arrhythmia               550,000      Ngbauji     5% \n
                 c. Chronic Kidney Disease  1,500,000    Atabrikang   15%\n
                 d. Diabetes                 800,000     Okorobilom   10%\n
                 e. Arthritis                450,000      Emeremen    10%\n");


        println!("Enter your name:");
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name:f32 = name.trim().parse().expect("Invalid string");

         println!("Enter your Date of Birth:");
        io::stdin().read_line(&mut dob).expect("Failed to read input");
        let dob:f32 = dob.trim().parse().expect("Invalid string");

        println!("Enter your Age:");
        io::stdin().read_line(&mut age).expect("Failed to read input");
        let age:f32 = age.trim().parse().expect("Invalid string");

         println!("Enter your Email address:");
        io::stdin().read_line(&mut email).expect("Failed to read input");
        let email = email.trim().parse().expect("Invalid string");

         println!("Enter your Phone number:");
        io::stdin().read_line(&mut phone_no).expect("Failed to read input");
        let phone_no:f32 = phone_no.trim().parse().expect("Invalid string");

         println!("Enter The number of Siblings you have:");
        io::stdin().read_line(&mut siblings_no).expect("Failed to read input");
        let siblings_no:f32 = siblings_no.trim().parse().expect("Invalid string");

         println!("Enter the nuber of children you have:");
        io::stdin().read_line(&mut children_no).expect("Failed to read input");
        let children_no:f32 = children_no.trim().parse().expect("Invalid string");

         println!("Enter letter assigned to your diagnosis:");
        io::stdin().read_line(&mut diagnosis).expect("Failed to read input");
        let diagnosis = diagnosis.trim().parse().expect("Invalid string");

         println!("Enter your village name:");
        io::stdin().read_line(&mut village_name).expect("Failed to read input");
        let village_name = village_name.trim().parse().expect("Invalid string");

        let cost_a:f32 = 1200000.0;
        let cost_b:f32 = 550000.0;
        let cost_c:f32 = 1500000.0;
        let cost_d:f32 = 800000.0;
        let cost_e:f32 = 450000.0;


        if diagnosis == a && age > 50.0 && children_no > 4.0 && village_name == "Akpabom" {
            let calc = cost_a * 0.80;
            println!("Your name is {}",name);
            println!("Your Date of Birth is {}", dob);
            println!("Your age is {}", age);
            println!("You have {} siblings", siblings_no);
            println!("You have {} children", children_no);
            println!("You have {} disease", diagnosis);
            println!("You are from {} village", village_name);
            println!("Yayyy, you received a discount!\n
                      Your total cost is {}", calc);break;
        }   else{
                println!("Your total cost is {}", cost_a);break;
            }

        
        else if  diagnosis == b  && age == 30.0 && siblings_no > 4.0 && village_name == "Ngbauji" {
            let calc = cost_b * 0.95;
            println!("Your name is {}",name);
            println!("Your Date of Birth is {}", dob);
            println!("Your age is {}", age);
            println!("You have {} siblings", siblings_no);
            println!("You have {} children", children_no);
            println!("You have {} disease", diagnosis);
            println!("You are from {} village", village_name);
            println!("Yayyy, you received a discount!\n
                      Your total cost is {}", calc);break;
        } else{
                println!("Your total cost is {}", cost_b);break;
            }

        
        else if  diagnosis == c && age > 40.0 && children_no > 3.0 && village_name == "Atabrikang" && siblings_no > 3.0 {
            let calc = cost_c * 0.85;
            println!("Your name is {}",name);
            println!("Your Date of Birth is {}", dob);
            println!("Your age is {}", age);
            println!("You have {} siblings", siblings_no);
            println!("You have {} children", children_no);
            println!("You have {} disease", diagnosis);
            println!("You are from {} village", village_name);
            println!("Yayyy, you received a discount!\n
                      Your total cost is {}", calc);break;
            }else{
                println!("Your total cost is {}", cost_c);break;
            }


        else if  diagnosis == d && age > 28.0 && age < 45.0 && children_no < 4.0 && children_no > 2.0 && village_name == "Okorobilom" {
            let calc = cost_d * 0.90;
            println!("Your name is {}",name);
            println!("Your Date of Birth is {}", dob);
            println!("Your age is {}", age);
            println!("You have {} siblings", siblings_no);
            println!("You have {} children", children_no);
            println!("You have {} disease", diagnosis);
            println!("You are from {} village", village_name);
            println!("Yayyy, you received a discount!\n
                      Your total cost is {}", calc);break;
            }else{
                println!("Your total cost is {}", cost_d);break;
            }

        
        else if  diagnosis == e  && age > 58.0 && children_no > 5.0 && siblings_no > 5.0  && village_name == "Emeremen" {
            {let calc = cost_e * 0.90;
            println!("Your name is {}",name);
            println!("Your Date of Birth is {}", dob);
            println!("Your age is {}", age);
            println!("You have {} siblings", siblings_no);
            println!("You have {} children", children_no);
            println!("You have {} disease", diagnosis);
            println!("You are from {} village", village_name);
            println!("Yayyy, you received a discount!\n
                      Your total cost is {}", calc);break;}
        } else {
                println!("Your total cost is {}", cost_a);break;
            }

    
    }


}
