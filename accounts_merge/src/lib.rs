//https://www.youtube.com/watch?v=ayW5B2W9hfo&ab_channel=PotatoCoders
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut merged = vec![];
        let mut email_to_index = HashMap::new();

        for account in accounts {
            let mut account = account.into_iter();
            let name = emails.next().unwrap();
            let mut emails = HashSet::new();
            let mut indices_to_merge = vec![];

            for email in account {
                if let Some(index) = email_to_index.get(&email) {
                    indices_to_merge.push(index);
                }
                emails.insert(email);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let accounts = vec![
            vec!["John", "johnsmith@mail.com", "john_newyork@mail.com"],
            vec!["John", "johnsmith@mail.com", "john00@mail.com"],
            vec!["Mary", "mary@mail.com"],
            vec!["John", "johnnybravo@mail.com"],
        ]
        .into_iter()
        .map(|strs| {
            strs.into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

        let expected = vec![
            vec![
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com",
            ],
            vec!["Mary", "mary@mail.com"],
            vec!["John", "johnnybravo@mail.com"],
        ]
        .into_iter()
        .map(|strs| {
            strs.into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
        let ans = Solution::accounts_merge(accounts);
        assert_eq!(expected, ans);
    }
}

struct Solution;
