module rpg_mmo::game_world { 
 
 use sui::object::{Self, UID};
 use sui::tx_context::{Self, TxContext};
 use sui::transfer;
 use sui::event;
 use std::sting::{Self, String};
 use std::vector;
 use sui::table::{Self, Table};

   //=============ESTRUTURA DE DADOS================//

   // Representa o mundo do jogo como um todo
   // este é um objeto compartilhado que todos pordem ver e interagir
    struct GameWorldd has key, store {
        
        id: UID,
        name: String,
        max_x: u64,
        max_y: u64,
        total_players: u64,
       locations: Table<u64, Location>,
       next_location_id:} u64,
    }  
   // Representa uma localização específica no mundo do jogo}
   
      struct Location has store {
        id: u64,
        name: String,
        description: String,
        x: u64,
        y: u64,
        location_type: u8, // 0: cidade, 1: floresta, 2: montanha, etc.
        level_requirement: u64, // nível mínimo para acessar
        has_shop: bool, // se tem loja para comprar itens
        has_inn: bool, // se tem estalagem para descansar
        // Lista de jogadores atualmente nesta localização
        players_present: vector<address>,
        // Lista de NPCs nesta localização
        npcs: vector
      }

    // Representa um NPC (personagem não jogável)