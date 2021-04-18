-- Your SQL goes here

CREATE TABLE `miasta` (
  `id` int(11) NOT NULL,
  `nazwa` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `ogloszenia` (
  `id` int(11) NOT NULL,
  `id_nadawca` int(11) NOT NULL,
  `nadawca` varchar(255) NOT NULL,
  `temat` varchar(255) NOT NULL,
  `widok_od` datetime NOT NULL,
  `widok_do` datetime NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `ogloszenia_tresc` (
  `id` int(11) NOT NULL,
  `id_ogloszenie` int(11) NOT NULL,
  `dane` mediumtext NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `uprawnienia` (
  `id` int(11) NOT NULL,
  `nazwa` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `uzytkownicy` (
  `id` int(11) NOT NULL,
  `login` varchar(255) NOT NULL UNIQUE,
  `imie` varchar(255) NOT NULL,
  `nazwisko` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `uzytkownicy_dane` (
  `id` int(11) NOT NULL,
  `id_uzytkownik` int(11) NOT NULL,
  `id_miasto` int(11) NOT NULL,
  `ulica` varchar(255) NOT NULL,
  `nr_domu` varchar(255) NOT NULL,
  `kod_pocztowy` varchar(255) NOT NULL,
  `pesel` varchar(255) NOT NULL,
  `nr_dowodu` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `uzytkownicy_hasla` (
  `id` int(11) NOT NULL,
  `id_uzytkownik` int(11) NOT NULL,
  `haslo` varchar(512) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `uzytkownicy_uprawnienia` (
  `id` int(11) NOT NULL,
  `id_uzytkownik` int(11) NOT NULL,
  `id_uprawnienie` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `wiadomosci` (
  `id` int(11) NOT NULL,
  `nadawca` varchar(255) NOT NULL,
  `temat` varchar(255) NOT NULL,
  `priorytet` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `wiadomosci_tresc` (
  `id` int(11) NOT NULL,
  `id_wiadomosc` int(11) NOT NULL,
  `dane` mediumtext NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `wiadomosci_uczestnicy` (
  `id` int(11) NOT NULL,
  `id_wiadomosc` int(11) NOT NULL,
  `id_uczestnik` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

ALTER TABLE `miasta`
  ADD PRIMARY KEY (`id`);

ALTER TABLE `ogloszenia`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_nadawca` (`id_nadawca`);

ALTER TABLE `ogloszenia_tresc`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_ogloszenie` (`id_ogloszenie`);

ALTER TABLE `uprawnienia`
  ADD PRIMARY KEY (`id`);

ALTER TABLE `uzytkownicy`
  ADD PRIMARY KEY (`id`);

ALTER TABLE `uzytkownicy_dane`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_uzytkownik` (`id_uzytkownik`),
  ADD KEY `id_miasto` (`id_miasto`);

ALTER TABLE `uzytkownicy_hasla`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_uzytkownik` (`id_uzytkownik`);

ALTER TABLE `uzytkownicy_uprawnienia`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_uzytkownik` (`id_uzytkownik`),
  ADD KEY `id_uprawnienie` (`id_uprawnienie`);

ALTER TABLE `wiadomosci`
  ADD PRIMARY KEY (`id`);

ALTER TABLE `wiadomosci_tresc`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_wiadomosc` (`id_wiadomosc`);

ALTER TABLE `wiadomosci_uczestnicy`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_uczestnik` (`id_uczestnik`),
  ADD KEY `id_wiadomosc` (`id_wiadomosc`);

ALTER TABLE `miasta`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `ogloszenia`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `ogloszenia_tresc`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `uprawnienia`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `uzytkownicy`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `uzytkownicy_dane`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `uzytkownicy_hasla`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `uzytkownicy_uprawnienia`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `wiadomosci`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `wiadomosci_tresc`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `wiadomosci_uczestnicy`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

ALTER TABLE `ogloszenia`
  ADD CONSTRAINT `ogloszenia_ibfk_1` FOREIGN KEY (`id_nadawca`) REFERENCES `uzytkownicy` (`id`);

ALTER TABLE `ogloszenia_tresc`
  ADD CONSTRAINT `ogloszenia_tresc_ibfk_1` FOREIGN KEY (`id_ogloszenie`) REFERENCES `ogloszenia` (`id`);

ALTER TABLE `uzytkownicy_dane`
  ADD CONSTRAINT `uzytkownicy_dane_ibfk_1` FOREIGN KEY (`id_uzytkownik`) REFERENCES `uzytkownicy` (`id`),
  ADD CONSTRAINT `uzytkownicy_dane_ibfk_2` FOREIGN KEY (`id_miasto`) REFERENCES `miasta` (`id`);

ALTER TABLE `uzytkownicy_hasla`
  ADD CONSTRAINT `uzytkownicy_hasla_ibfk_1` FOREIGN KEY (`id_uzytkownik`) REFERENCES `uzytkownicy` (`id`);

ALTER TABLE `uzytkownicy_uprawnienia`
  ADD CONSTRAINT `uzytkownicy_uprawnienia_ibfk_1` FOREIGN KEY (`id_uzytkownik`) REFERENCES `uzytkownicy` (`id`),
  ADD CONSTRAINT `uzytkownicy_uprawnienia_ibfk_2` FOREIGN KEY (`id_uprawnienie`) REFERENCES `uprawnienia` (`id`);

ALTER TABLE `wiadomosci_tresc`
  ADD CONSTRAINT `wiadomosci_tresc_ibfk_1` FOREIGN KEY (`id_wiadomosc`) REFERENCES `wiadomosci` (`id`);

ALTER TABLE `wiadomosci_uczestnicy`
  ADD CONSTRAINT `wiadomosci_uczestnicy_ibfk_1` FOREIGN KEY (`id_uczestnik`) REFERENCES `uprawnienia` (`id`),
  ADD CONSTRAINT `wiadomosci_uczestnicy_ibfk_2` FOREIGN KEY (`id_wiadomosc`) REFERENCES `wiadomosci` (`id`);