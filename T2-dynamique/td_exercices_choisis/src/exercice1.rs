/// Représente un morceau de musique avec une note et un titre.
pub struct Song {
    rank: u32,
    title: String,
}

/// Retourne la note moyenne d'un ensemble de morceaux.
pub fn average_rank(songs: &Vec<Song>) -> f64 {
    //TEST UTTILISER UN ITERABLE et collect exactement comme suggéré par GPT
    if songs.is_empty() {
        panic!("not implemented") ;
        //return 0.0;
    }
    let n =  songs.len() ;

    let mut somme :f64 = 0.0 ;
    for i in 0..n {
        somme += (songs[i].rank) as f64
    }

    return somme / (n as f64 );
}

/// Filtre les morceaux dans `songs` et ne garde que ceux dont la note est
/// strictement supérieure à `rank_min`.
pub fn filter_songs(songs: Vec<Song>, rank_min: u32) -> Vec<Song> {
    let mut filtered = Vec::new();

    for song in songs {
        if song.rank > rank_min {
            filtered.push(song);
        }
    }

    filtered
}


/// Filtre les morceaux dans `songs` pour ne conserver que ceux dont la note
/// est strictement supérieure à la moyenne.
pub fn good_songs(songs: Vec<Song>) -> Vec<Song> {
    let moyenne = average_rank(&songs);
    let mut filtered = Vec::new();

    for song in songs {
        if (song.rank) as f64 >= moyenne {
            filtered.push(song) ;
        }
    }

    filtered
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<Song> {
        vec![
            Song {
                rank: 4,
                title: String::from("Stairway to Heaven"),
            },
            Song {
                rank: 2,
                title: String::from("Never Gonna Give You Up"),
            },
            Song {
                rank: 5,
                title: String::from("Nigerian Marketplace"),
            },
        ]
    }

    #[test]
    #[should_panic]
    fn empty() {
        let sgs = Vec::new();
        let _m = average_rank(&sgs);
    }

    #[test]
    fn moyenne_un_seul_morceau() {
        let sgs = vec![Song {
            rank: 4,
            title: String::from("Stairway to Heaven"),
        }];
        let m = average_rank(&sgs);
        assert_eq!(m, 4.0);
    }

    #[test]
    fn moyenne_plusieurs_morceau() {
        let sgs = example();
        let m = average_rank(&sgs);
        assert!((m - 3.66666).abs() < 0.001);
    }

    #[test]
    fn filtre() {
        let sgs = example();
        let f = filter_songs(sgs, 4);
        assert!(f.len() == 1);
        assert_eq!(f[0].rank, 5);
        assert_eq!(f[0].title, "Nigerian Marketplace");
    }

    #[test]
    fn meilleurs() {
        let sgs = example();
        let f = good_songs(sgs);
        assert!(f.len() == 2);
        for m in &f {
            assert!(m.rank >= 4);
        }
    }
}
