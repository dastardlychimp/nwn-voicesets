# SQL Tables

## Voiceset Table (Compilation of soundsets)
| id | name |
|----|------|
|1   |my_voiceset|

## Soundset/Voiceset Table (Ties together voiceset to multiple soundsets)
| id | id_voiceset | id_soundset |
|----|--------------|--------------|
|1   | 1            | 3            |
|2   | 1            | 4            |

## Soundset Table
| id | name |
|----|------|
|3   |soundsetname|
|4   |soundset2|

## Sounds Table
| id | id_soundset  | idx_soundset  | name                    | text              | sound_data |
|----|--------------|---------------|-------------------------|-------------------|------------|
|1   | 3            | 0             | "my_attack_thing"       | "Attack!"         | binary     |
|2   | 3            | 1             | "sound_file_2"          | Null              | binary     |
|3   | 4            | 0             | "ss2_aatck"             | "Die you fiends!" | binary     |
