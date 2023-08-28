# CHANGELOG

# Historique des modifications

### TODO :
- ajout csrf -> pas évident
- question AtomicPtr::fetch_update : la mémoire de l'ancien pointeur sera-t-elle libérée après la fin des requests en cours ?
- static files
- moteur macro https://docs.rs/subst/latest/subst/
- intégration jointure

0.8.0 - 28 août 2023
- `changed` modules renommés service router lexicer cruder
- `added` début message flash maison

0.7.1 - 26 août 2023
- `added` RUST_LOG dans <fichier environnement>.conf
- `added` view.limit_sql
- `added` read sql traitement des jointures et _element
- `added` sqlic.crud_read_all

0.7.0 - 25 août 2023
- `added` ajout lecture des données de l'application et présentation dans une vue
- `added` ajout moteur sqlite pour les calculs du framework

0.6.1 - 22 août 2023
- `added` ajout tableid viewid formid elid dans les structures adéquates
- `added` load des forms

0.6.0 - 22 août 2023
- `added` template Tera remplace Askama

0.5.5 - 20 août 2023
- `added` template Askama avant bascule sur Tera

0.5.4 - 18 août 2023
- `fixed` AppState créé avant httpserver

0.5.3 - 18 août 2023
- `changed` tpl avec objet portail et application
- `added` tpl application

0.5.2 - 17 août 2023
- `added` gestion erreurs chargement du lexique

0.5.1 - 16 août 2023
- `added` lexic partagé unique

0.5.0 - 16 août 2023
- `changed` build-release.sh à la racine
- `changed` dockerfile mini avec seulement l'exécutable
- `added` lexic racine à portail et applications
- `added` template tpl_base

0.4.1 - 14 août 2023
- `added` development.conf production.conf externe à la webapp
- `changed` nom de baptème du projet SILEX Système d'Information LEXical (utilisation d'un lexique)

0.4.0 - 13 août 2023
- `added` docker pour test en https
- `changed` nom de baptème du projet SILEX Système d'Information LEXical (utilisation d'un lexique)

0.3.1 - 11 août 2023
- `added` formulaire login - redirection

0.3.0 - 11 août 2023
- `added` middleware session data
- `changed` nom des modules lexic routic servic

0.2.0 - 9 août 2023
- `added` découpage en modules
- `added` partage du pool et portail dans AppState

0.1.3 - 4 août 2023
- `changed` le nom du package redevient "rustic" car rustix existe déja dans crate.io
- `added` test pointeur partagé sur portail inter threads - pas concluant

0.1.2 - 3 août 2023
- `added` item dans template - dictionnaire accessible par l'application

0.1.1 - 3 août 2023
- `added` actix-web, template askama

0.1.0 - 1er août 2023
- `added` initialisation dans github avec des bricoles

###### Types de changements:
`added` *pour les nouvelles fonctionnalités.*
`changed` *pour les changements aux fonctionnalités préexistantes.*
`deprecated` *pour les fonctionnalités qui seront bientôt supprimées*.
`removed` *pour les fonctionnalités désormais supprimées.*
`fixed` *pour les corrections de bugs.*
`security` *en cas de vulnérabilités.*