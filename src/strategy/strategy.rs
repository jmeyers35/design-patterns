// We'll use a video game enemy as an example of the strategy pattern. An Enemy can attack which
// returns a damage value, which is calculated by doing some arithmetic on the stats of the Player
// and the Enemy. We'll encapsulate this calculation in an Attack strategy.

// Stats are a character's statistics.

#[derive(Copy, Clone)] // I didn't want to deal with lifetimes in the Attack implementations.
pub struct Stats {
    level: u16,
    strength: u16,
    defense: u16,
}

pub trait Enemy {
    fn calc_attack() -> f64;
    fn get_stats() -> Stats;
}

// Attack encapuslates the ability to do an Attack.
pub trait Attack {
    fn calc_damange(&self) -> f64;
    fn set_attacker(&mut self, stats: Stats);
    fn set_defender(&mut self, stats: Stats);
}

// Attacker's strength less the defender's defense, scaled by level.
pub struct SimpleAttack {
    attacker_stats: Stats,
    defender_stats: Stats,
}

impl Attack for SimpleAttack {
    fn calc_damange(&self) -> f64 {
        Into::<f64>::into(self.attacker_stats.strength * (1 + self.attacker_stats.level / 100))
            - Into::<f64>::into(self.defender_stats.defense * (1 + self.defender_stats.level / 100))
    }

    fn set_attacker(&mut self, stats: Stats) {
        self.attacker_stats = stats
    }

    fn set_defender(&mut self, stats: Stats) {
        self.defender_stats = stats
    }
}

// Attacker's strength less the defenders defense, but defense is scaled down relative to attacker's
// level.
pub struct PenetratingAttack {
    attacker_stats: Stats,
    defender_stats: Stats,
}

impl Attack for PenetratingAttack {
    fn calc_damange(&self) -> f64 {
        let effective_defense = self.defender_stats.defense * (1 - self.attacker_stats.level / 250);

        Into::<f64>::into(self.attacker_stats.strength * (1 + self.attacker_stats.level / 100))
            - Into::<f64>::into(effective_defense * (1 + self.defender_stats.level / 100))
    }

    fn set_attacker(&mut self, stats: Stats) {
        self.attacker_stats = stats
    }

    fn set_defender(&mut self, stats: Stats) {
        self.defender_stats = stats
    }
}
