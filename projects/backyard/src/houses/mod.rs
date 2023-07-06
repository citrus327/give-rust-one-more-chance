pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {
            super::serving::take_payment();
        }

        fn serve_order() {}

        fn take_payment() {
            self::serve_order();
        }

        pub mod testing {
            fn test_fn() {
                super::serve_order();
            }
        }
    }
}

fn deliver_food() {}
