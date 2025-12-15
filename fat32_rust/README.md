# FAT32 Rust Reimplementation

## Description

Ce projet est une réimplémentation partielle du système de fichiers **FAT32** en Rust.  
Il a été réalisé dans un contexte académique et vise à comprendre et manipuler les structures internes d’un système de fichiers réel.

Le projet permet actuellement :
- la lecture du **Boot Sector**
- le calcul du **layout FAT32**
- la lecture de la **FAT**
- la lecture de **clusters**
- la lecture de **répertoires**
- la lecture de **fichiers**
- une navigation logique interne permettant de retrouver des fichiers à partir de leur emplacement

Les fonctionnalités de création et d’écriture de fichiers ne sont pas encore implémentées.

---

## Contraintes du projet

- Le projet est **`no_std`**
- Le crate `alloc` est utilisé lorsque nécessaire
- Le code est compatible avec un environnement **Linux**
- Le projet est structuré comme une bibliothèque Rust
- Des **tests unitaires** sont fournis pour les briques principales

---

## Architecture

Le projet est découpé en modules clairs :

- `disk` : abstraction d’un disque bloc
- `boot_sector` : parsing du secteur de boot FAT32
- `layout` : calcul de la position des structures FAT32
- `fat` : lecture de la File Allocation Table
- `cluster` : lecture de clusters à partir du disque
- `file` : lecture de fichiers via la FAT
- `dir` : lecture et parsing des répertoires FAT32
- `path` : logique interne de résolution de chemins (utilisée par les couches supérieures)

Chaque composant est testé indépendamment afin de limiter les effets de bord.

---

## Tests

Les tests couvrent notamment :
- le parsing du boot sector
- le calcul du layout FAT32
- la lecture de la FAT
- la lecture de clusters
- la lecture de fichiers sur plusieurs clusters
- la lecture de répertoires

Les tests utilisent un disque factice en mémoire afin de simuler un environnement FAT32 contrôlé.

---

## Limitations connues

- Les **Long File Names (LFN)** ne sont pas supportés
- L’écriture et la création de fichiers ne sont pas implémentées
- L’interface utilisateur (CLI) n’est pas fournie
- Certains comportements avancés de FAT32 ne sont pas gérés (fragmentation complexe, volumes corrompus, etc.)

---

## Utilisation d’outils d’assistance

Ce projet a été réalisé **principalement de manière individuelle**.  
Une **aide ponctuelle par outil d’assistance IA** a été utilisée pour :
- clarifier certains concepts théoriques (FAT32, organisation des structures)
- aider au débogage de problèmes complexes
- améliorer la lisibilité et la structure du code

Toutes les décisions finales, l’implémentation et la validation du code ont été effectuées par moi même.

---

## Auteur

Rakulan SIVATHASAN
