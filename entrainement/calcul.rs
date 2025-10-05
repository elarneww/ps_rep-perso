fn main(){
    // nom variable :type =
    let reel : f32 = 3.5 ;
    for i in 1..11 {
        //print ne met pas \n, contrairement à println
        print!("{ } ,x{}\r",reel* i as f32,i) ;//as f32 = cast de i, réel * integer... not exist 
    }

    // Essais avec poitneurs/references :
    let mut reel: f32 = 3.5;

    {//scope 1 pour nettoyer ref non mutable 'r'
        let r: &f32 = &reel;
        println!("lecture via r: {}", *r);
    } // r n’existe plus ici : fin du borrowing

    {// scope2 : on peux maintenant borrow reel à nouveau
        let p: &mut f32 = &mut reel;
        *p = 4.2;
        println!("modification via p: {}", *p);
    }
    
}