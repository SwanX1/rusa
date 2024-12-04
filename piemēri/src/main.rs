rusa::rūsa! {
    funkcija galvenā() {
        ļaut mainīgam x = 1;

        atbilst x {
            42 => {
                drukātlīn!("kaut kas sapisās!")
            }
            _ => drukātlīn!("aiziet")
        }

        katram i iekš 0..10 {
            kamēr x < i {
                x += 1;
            }
        }

        //otra();
    }

    #[atļaut(nesasniedzams_kods)]
    funkcija otra() {
        panikot!("ak, nē"); // Literāri pareizajiem
        bļe!("kas nahuj notika??"); // Parastam latvietim
    }
}
