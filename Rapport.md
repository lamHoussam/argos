# Projet Sécurité des logiciels GLO-4009/GLO-7009

# Résumé

Outil de détection de buffer overflows dans le code source d'un programme C ainsi que dans un binaire compilé. L'outil peut aussi proposer des correctifs pour les vulnérabilités détectées dans le code source, et aussi détecter les memory leaks dans les binaires compilés.

L'outil est composé de deux parties: un analyseur statique et un analyseur dynamique. L'analyseur statique analyse le code source du programme pour détecter les patterns de buffer overflows et proposer des correctifs. L'analyseur dynamique analyse le binaire compilé pour détecter les buffer overflows et les memory leaks.

# Introduction

- Description du problème
- Description de l'objectif du projet

# Structure du projet

```bash
src/
├── intercept.c
├── libintercept.so
├── lib.rs
├── main.rs
└── parser.md
test/
├── main
└── main.c
```

# Analyse statique

- Description de l'analyse statique
- Description des patterns de buffer overflows détectés
- Description des correctifs proposés

# Analyse dynamique

## Description des shared libraries (LD_PRELOAD)

Nous utilisons la variable d'environnement `LD_PRELOAD` pour charger notre librairie dynamique avant les autres librairies dynamiques. Cela nous permet d'intercepter les appels aux fonctions de la libc et de les rediriger vers nos propres fonctions.

## Description des librairies dynamiques dans Rust


## Description des IPCs (shared memory)



