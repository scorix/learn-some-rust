#[macro_use]
extern crate mysql;

#[derive(Debug)]
struct User {
    id: Option<i32>,
    name: String,
    nick: Option<String>
}

fn create_user_table(pool: &mysql::Pool) {
    pool.prep_exec(r"CREATE TEMPORARY TABLE `tmp`.`users` (
        `id` int not null auto_increment,
        `name` varchar(24) not null,
        `nick` varchar(24),
        PRIMARY KEY (`id`)
    )", ()).unwrap();
}

fn main() {
    let pool = mysql::Pool::new("mysql://root:root@192.168.203.135:31857").unwrap();
    create_user_table(&pool);

    for mut stmt in pool.prepare(r"INSERT INTO `tmp`.`users` (`name`, `nick`) VALUES (:name, :nick)").into_iter() {
        let user1 = User { id: None, name: "Bob".into(), nick: None };
        stmt.execute(params!{
            "name" => mysql::Value::from(user1.name),
            "nick" => mysql::Value::from(&user1.nick),
        }).unwrap();

        stmt.execute(params!{
            "name" => mysql::Value::from("Yu Zhang"),
            "nick" => mysql::Value::from("Scorix"),
        }).unwrap();
    };

    let users : Vec<User> = pool.prep_exec("SELECT * FROM `tmp`.`users`", ()).map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, nick) = mysql::from_row(row);
            User { id: id, name: name, nick: nick }
        }).collect()
    }).unwrap();

    println!("{:?}", users);

    let count : i32 = pool.prep_exec("SELECT COUNT(*) FROM `tmp`.`users`", ()).map(|mut result| {
        mysql::from_row(result.next().unwrap().unwrap())
    }).unwrap();

    println!("There are {:?} users in the database.", count);
}
