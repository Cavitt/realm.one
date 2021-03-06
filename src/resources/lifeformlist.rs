#![allow(dead_code)]
use amethyst::{
    core::{Transform},
};

use crate::components::{LifeformComponent, LifeformType};
use std::collections::HashMap;
use std::net::SocketAddr;

// Couple of biz guys, remember this
pub struct LifeformList {
    pub list: Vec<Option<LifeformComponent>>,
    ips: HashMap<SocketAddr, usize>,
    ids: HashMap<u64, usize>,
    players: HashMap<String, Vec<u64>>,  // Players in a room
    monsters: HashMap<String, Vec<u64>>, // Monsters in a room
    index: usize,
}

impl Default for LifeformList {
    fn default() -> Self {
        LifeformList::new() 
    }
}

impl LifeformList {
    pub fn new() -> Self {
        Self {
            list: Vec::<Option<LifeformComponent>>::new(),
            ips: HashMap::<SocketAddr, usize>::new(),
            ids: HashMap::<u64, usize>::new(),
            players: HashMap::<String, Vec<u64>>::new(),  // Players in a room
            monsters: HashMap::<String, Vec<u64>>::new(), // Monsters in a room
            index: 0 as usize, 
        }
    }

    pub fn add(&mut self, lifeform: LifeformComponent) {
        match lifeform.ip {
            Some(ip) => self.ips.insert(ip, self.index),
            None => None,
        };

        // Add the index to the hashmaps for room lookup
        match lifeform.kind {
            LifeformType::Player  => {
                match self.players.get_mut(&lifeform.room) {
                    Some(vec) => vec.push(lifeform.id()),
                    None => {
                        let mut new_room = Vec::<u64>::new();
                        new_room.push(lifeform.id());
                        self.players.insert(lifeform.room.clone(), new_room);
                    }
                };
            },
            LifeformType::Monster=> {
                match self.monsters.get_mut(&lifeform.room) {
                    Some(vec) => vec.push(lifeform.id()),
                    None => {
                        let mut new_room = Vec::<u64>::new();
                        new_room.push(lifeform.id());
                        self.monsters.insert(lifeform.room.clone(), new_room);
                    }
                };
            },
            _ => (),
        }

        self.ids.insert(lifeform.id(), self.index); 
        self.list.push(Some(lifeform));
        self.index += 1;
    }
    
    fn remove(&mut self, slice: usize) {
        self.list[slice] = None;
    }
    
    pub fn remove_with_ip(&mut self, ip: SocketAddr) {
        self.remove(*self.ips.get(&ip).unwrap()); 
    }

    pub fn remove_with_id(&mut self, id: u64) {
        self.remove(*self.ids.get(&id).unwrap()); 
    }
    
    pub fn get_from_ip(&mut self, ip: SocketAddr) -> Option<LifeformComponent> {
        self.list[*self.ips.get(&ip).unwrap()].clone()
    }
    
    /// Get all the IPs in a certain room
    pub fn ip_in_room(&mut self, room: &String) -> Vec<SocketAddr> {
        let mut ip = Vec::<SocketAddr>::new();

        for lifeform in &self.list {
            match lifeform {
                Some(lf) => {
                    if lf.room == *room && lf.kind == LifeformType::Player {
                        ip.push(lf.ip());
                    }
                },
                None => {}
            }
        }
        ip
    }
    
    /// Get all the players in a room
    pub fn in_room(&self, room: &String, kind: LifeformType) -> Option<&Vec<u64>> {
        match kind {
            LifeformType::Player  => self.players.get(room),
            LifeformType::Monster => self.monsters.get(room),
            _ => None,
        }
    }
   
    pub fn get_from_id(&self, id: u64) -> Option<LifeformComponent> {
        self.list[*self.ids.get(&id).unwrap()].clone()
    }

    

    pub fn replace(&mut self, player: LifeformComponent) {
        let id = player.id(); 
        self.list[*self.ids.get(&id).unwrap()] = Some(player); 
    }

    pub fn get_from_transform(&self, tr: Transform) -> Option<LifeformComponent> {
        for player in self.list.iter() {
            match player {
                Some(pl) => {
                    if pl.trans().translation() == tr.translation() {
                        return Some(pl.clone());
                    } 
                },
                None => (),
            }
        }
        None
    }
}

impl Iterator for LifeformList {
    type Item = LifeformComponent;
    
    fn next(&mut self) -> Option<Self::Item> {
        let i = 0;

        while i < self.index {
            if self.list[i].is_some() {
                return self.list[i].clone();
            }
        }
        None
    }
}
