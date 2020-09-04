# Testing manually

## sound init but no sound played
+ cargo run
+ press ESC

program exits without error

## play sound using `RaylibAudio.play_sound`
+ cargo run
+ press keyboard down one or more times
+ press ESC

program exits without error

## play sound using `RaylibAudio.play_sound_multi`
+ cargo run
+ press keyboard up one or more times
+ press ESC

program exits with STATUS_HEAP_CORRUPTION or STATUS_ACCESS_VIOLATION
