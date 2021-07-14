fn main() {
	let a = Employee{
	   company:String::from("Google"),
	   name:String::from("John"),
	   age:50
	};
	let b = Employee {
	   company:String::from("Google"),
	   name:String::from("Kannan"),
	   age:32
	};
	let elder = who_is_elder(a,b);
	println!("elder is:");

	display(elder);
 }
 fn who_is_elder (a:Employee,b:Employee)->Employee {
	if a.age>b.age {
	   return a;
	} else {
	   return b;
	}
 }
 fn display(emp:Employee) {
	println!("Name is :{} company is {} age is {}",emp.name,emp.company,emp.age);
 }
 struct Employee {
	name:String,
	company:String,
	age:u32
 }