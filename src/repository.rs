use prost::Message;
use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;

use crate::generated::models::{Account, Todo};

const ACCOUNT_PREFIX: &str = "account:";
const TODO_PREFIX: &str = "todo:";

pub trait Repository<T> {
    fn store(&mut self, entity: &T) -> Result<(), Box<dyn Error>>;
    fn get_by_id(&self, id: &str) -> Result<Option<T>, Box<dyn Error>>;
    fn delete(&mut self, id: &str) -> Result<bool, Box<dyn Error>>;
}

pub struct InMemoryRepo {
    store: HashMap<String, Vec<u8>>,
}

impl InMemoryRepo {
    pub fn new() -> Self {
        InMemoryRepo {
            store: HashMap::new(),
        }
    }
}

impl Repository<Account> for InMemoryRepo {
    fn store(&mut self, account: &Account) -> Result<(), Box<dyn Error>> {
        let mut buf = Vec::new();
        prost::Message::encode(account, &mut buf)?;

        let key = format!("{}{}", ACCOUNT_PREFIX, account.name);
        self.store.insert(key, buf);
        return Ok(());
    }

    fn get_by_id(&self, name: &str) -> Result<Option<Account>, Box<dyn Error>> {
        let key = format!("{}{}", ACCOUNT_PREFIX, name);
        let rsp = self.store.get(&key);
        return match rsp {
            Some(vec) => {
                let account = Account::decode(Cursor::new(vec))?;
                Ok(Some(account))
            }
            None => Ok(None),
        };
    }

    fn delete(&mut self, name: &str) -> Result<bool, Box<dyn Error>> {
        let key = format!("{}{}", ACCOUNT_PREFIX, name);
        let result = self.store.remove(&key);
        return Ok(result.is_some());
    }
}

impl Repository<Todo> for InMemoryRepo {
    fn store(&mut self, todo: &Todo) -> Result<(), Box<dyn Error>> {
        let mut buf = Vec::new();
        prost::Message::encode(todo, &mut buf)?;

        let key = format!("{}{}", TODO_PREFIX, todo.id);
        self.store.insert(key, buf);
        return Ok(());
    }

    fn get_by_id(&self, id: &str) -> Result<Option<Todo>, Box<dyn Error>> {
        let key = format!("{}{}", TODO_PREFIX, id);
        let rsp = self.store.get(&key);
        return match rsp {
            Some(vec) => {
                let todo = Todo::decode(Cursor::new(vec))?;
                Ok(Some(todo))
            }
            None => Ok(None),
        };
    }

    fn delete(&mut self, id: &str) -> Result<bool, Box<dyn Error>>  {
        let key = format!("{}{}", TODO_PREFIX, id);
        let result = self.store.remove(&key);
        return Ok(result.is_some());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inmemory_repo_account_delete_exists() {
        let mut base_repo = InMemoryRepo::new();
        base_repo.store.insert(format!("{}{}", ACCOUNT_PREFIX, "1234"), vec![1]);
        let repo = &mut base_repo as &mut dyn Repository<Account>;
        
        let actual = repo.delete("1234");

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), true);
    }

    #[test]
    fn inmemory_repo_account_delete_doesnt_exist() {
        let repo = &mut InMemoryRepo::new() as &mut dyn Repository<Account>;
        
        let actual = repo.delete("1234");

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), false);
    }

    
    #[test]
    fn inmemory_repo_todo_delete_exists() {
        let mut base_repo = InMemoryRepo::new();
        base_repo.store.insert(format!("{}{}", TODO_PREFIX, "1234"), vec![1]);
        let repo = &mut base_repo as &mut dyn Repository<Todo>;
        
        let actual = repo.delete("1234");

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), true);
    }

    #[test]
    fn inmemory_repo_todo_delete_doesnt_exist() {
        let repo = &mut InMemoryRepo::new() as &mut dyn Repository<Todo>;
        
        let actual = repo.delete("1234");

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), false);
    }
}
