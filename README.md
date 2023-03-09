
# Screening exercise for XXXX

You have a list of players with their ages et scores (ratio);
You must extract from the list: the champions

A player is declared champion, if no one else eliminates them.
Meaning:

* No one is strictly stronger, and younger or same age and no one is both younger and both stronger or same score

your mission: in the language of your choice, code the function allowing to find the champions of the list


We will regard following points with attention:

* Exactfulness of the results : candidate has thought of overall logic and edge cases ?
* Performance: How does the algorithm behaves when the list of players grows ?
* Clarity and readibility of the code

# Input -> Output
[{ ratio: 1000, age: 30, name: "Moses" },{ ratio: 1100, age: 29, name: "Karl" },{ ratio: 1200, age: 31, name: "Lebron" },] => [{ ratio: 1100, age: 29, name: "Karl" },{ ratio: 1200, age: 31, name: "Lebron" },]