# SystemObslugiStudentow

Nowy wspaniały projekt.

## Jak rust to ogarnąć wypada
* Coś do nauczenia się rusta xD
	* https://www.rust-lang.org/learn
	* https://doc.rust-lang.org/beta/
* Coś do obsługi baz danych
	* https://diesel.rs/
	* https://github.com/perdumonocle/sql-builder
* Coś do zrobienia gui
	* https://github.com/linebender/druid
	* https://github.com/MoAlyousef/fltk-rs
	* https://github.com/woboq/qmetaobject-rs
	* https://dev.to/davidedelpapa/rust-gui-introduction-a-k-a-the-state-of-rust-gui-libraries-as-of-january-2021-40gl
* Jeden z lepszych guide jak zrobić api w rust
	* https://www.youtube.com/watch?v=VMZdGX9wC4g

## Baza danych do testów
* XAMPP: https://www.apachefriends.org/pl/index.html
	* Zintegrowany webserwer z aplikacją do zarządzania mysql (phpMyAdmin)
	* Ew zwykła instalacja mySQL + jakiś tool do zarządzania
* Zaimportowanie sos.sql (docelowo będzie *kiedyś* zawierał właściwą bazę danych)

## Do testowania api
* Postman
	* https://www.postman.com/
	* eit-pa-sos, invite: https://app.getpostman.com/join-team?invite_code=631ae24d11a891b8cdf23269d6fbe396&ws=08dea26f-2149-4142-8872-973fd783a4cd

## Gdyby był problem z bibliotekami dla diesela
* Zainstalować: https://downloads.mysql.com/archives/c-c/
* Dodać zmienną środowiskową MYSQLCLIENT_LIB_DIR wskazującą na C:\Program Files\MySQL\MySQL Connector C 6.1\lib\vs14

## Gdyby wymagało builda nightly rusta
* CMD: rustup toolchain install nightly
* CMD: rustup default nightly

## cargo-make
* CMD: cargo install cargo-make
* CMD: cargo make -V

## Założenia projektowe

* Zewnętrzna (?wewnętrzna - sqlite???) baza danych - praca aplikacji na zasadzie nakładki do bazy.
	* Dzięki temu możliwość przepisania systemu w przyszłości / dodania appki mobilnej
* Docelowo postawienie bazy danych na jakimś vpsie/chmurze
* System magazynowy / sprzedażowy / jsos 3.0? Zakładana realizacja:
	* Podsystemu użytkowników (zarządzanie z uprawnieniami etc.)
		* W takim razie też może księgowość/wypłaty?
		* Podsystem HR?
	* Podsystemu wiadomości
	* Podsystemu z danymi (magazyn/zamówienia/kursy/zapisy etc.)
		* Zarządzanie magazynem (ew zasobami np kursami)
		* Tworzenie czegoś
		* Usuwanie czegoś
		* Edycja czegoś
* (Ew. aplikacja mobilna może być po prostu rozszerzeniem niektórych funkcji:)
	* Przy tworzeniu czegoś możliwość wprowadzenia "kodu produktu" zczytując qr/barcode

<br><br>
UML na miarę naszych możliwości: <br><br>
![](docs/nibyuml.png)

<br><br>
A tak wygląda **wstępna koncepcja** bazy danych: <br><br>
![](docs/koncepcja_bazy_danych.png)
