Un project de stage qui a pour bute de metre en avant une boulangerie et ses produit.

Le project et composé de 2 partie. Une partie vitrine qui permet de metre en avant les produit. Et une partie shop/cart qui permet de prendre des commande depuis le site.
Les commande faite depuis le site seront a rétirer et payé en main propre directment en magasin.

La structure et MVC et l'intéraction entre client / server et est fait pour coroboré un maxmimum avec restfull.

Pour les répertoires principaux on a:

html:
    partials: les template html qui ne sont pas des vue en tant que tel mais des bout.
    static: Les templte qui n'ont pas besoins de context autre que celui des route pour pouvoir et rendu.
    views: Les templte complexe qui sont appelé pour faire les page dynamique du site.
migrations:
    composé des migration qui permete de réaliser les commande up et down de diesel pour gérer la base de donné

public:
    un répertoire public ou tout ce qu'y si trouve peut y être acceder via n simple liens. on y retrouve js/ css/
    images/: est réserver pour les image utilisé et placer par le developer du site
    uploads/ les fichier comme les fichiers image uploade par l'administrateur depuis le site. Ex: ajout d'un produit


SQL:
    un dossier ou l'ont place les fichier SQL ayant pour but de réaliser des actions sur la BD hors mais qu'y n'est pas appeler par le code Ex: seeders.


src:
    La ou toute la logique rust est placé 

    controllers/:
        C de MVC c'est le fichier ou se passe tout la logique de génération des page.

    models/:
        M de mvc c'est la ou on place tout ce qui a un lien avec la base de donné (hors exeption src/schema.rs) tout manipulation de la base doit être ici

        complex_request.rs:
            ce fichier permet d'y placer les requète non trivial qui sort des donné de plsieurs table et qui donc ne peuvent être facilment être place dans un 
            ***_models.rs. Ce fichier pourrait être amener à être fragmenter si celui devient trop gros.

    midlewares/:
        place des middleware qui permet la gestion des authentification et gestion des droits.
    
    macros.rs:
        déclaration des macro pour les centralisé

    route.rs:
        déclaration des route du site en statique que l'ont utilise en constant à travers le site ainsi qu'un context Tera des route .

    schama.rs:
        ce fichier permet de faire le lien entre les type propre à la base de donné et celui de rust il agit de binding

    statics.rs:
        Declaration de tout les object static qui seront partagé potentielement à travers tout l'app. Ex: Moteur TERA, ou le Writer pour les log

    utilities:
        Un fichier "utilitaires" ou on y définie des méthode, actions, trait  qui sont utilisé à travers toute l'app.

    main:
        declaration de l'app établisemement de l'env ajoute de route middleware etc.

.env:
    Un fichier utilisé pour y placé des variable extern pour l'app.Ex: Mode: prod/dev ou PORT: 80/8080 etc  

resetup.sh:
    ce fichier permet de réinitialiser et re-seeder la base de donné lors de changement.