# CHANGELOG

# Historique des modifications

### TODO :
- ajout csrf -> pas évident
- question AtomicPtr::fetch_update : la mémoire de l'ancien pointeur sera-t-elle libérée après la fin des requests en cours ?
- static files
- bouton coller le presse-papier
- arg nécessaire dans l'url si view dans un formulaire pour mémoriser le retour

0.18.0 - 23 septembre 2023
- `added` search : recherche globale

0.17.0 - 22 septembre 2023
- `added` filters ok

0.16.0 - 19 septembre 2023
- `added` onglets view dans le footer
- `added` style class sqlite sur les tr
- `added` tri des colonnes

0.15.0 - 16 septembre 2023
- `added` portail & view stylisés

0.14.0 - 16 septembre 2023
- `added` ajout feuille de style fomantic - jquery - nombreux fichiers
- `added` route /static /lexic
- `added` page portail stylisée

0.13.0 - 11 septembre 2023
- `changed` formulaire de visualisation obligatoire pour visualiser un article
- `changed` erreurs retournées au controlleur qui se chargera de les publier dans le flash_message
- `added` suppression d'un article


0.12.0 - 6 septembre 2023
- `deleted` actix-web-flash-messages qui ne fonctionnait pas
- `added` ajout du flashMessage dans la session

0.11.3 - 5 septembre 2023
- `changed` messages gérés par actix-web-flash-messages - non concluant

0.11.2 - 5 septembre 2023
- `changed` messages stockés dans les extensions de la requête - non concluant car non transmis dans la response

0.11.1 - 3 septembre 2023
- `added` renommage des mod et fn

0.11.0 - 3 septembre 2023
- `added` update d'un article

0.10.0 - 2 septembre 2023
- `added` boutons view edit add
- `added` macro template pour afficher les éléments
- `added` messages dans la session pour récupérer tous les messages créés dans les contrôleurs

0.9.2 - 1er septembre 2023
- `added` boutons view edit add
- `added` macro template pour afficher les éléments

0.9.1 - 31 août 2023
- `deleted` table.setting.col_display supprimé
- `added` template view dans le formulaire form_view
- `added` macro template pour afficher les éléments

0.9.0 - 30 août 2023
- `added` calcul des propriétés des élements

0.8.1 - 28 août 2023
- `added` view d'un enregistrement

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

## Évolution du distionnaire / Beedule
- type_element type_view type_form
- remplacer les noms de champs xxx-yyy en xxx_yyy
- table.setting.col_display supprimé
- view.form_view nécessaire pour visualiser un enregistrement
- si form_view lien vers form_edit si défini
- suppression element.compute_sqlite
- form_view obligatoire pour visualiser un enregistrement
- view action hide supprimée
