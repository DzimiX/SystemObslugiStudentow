-- phpMyAdmin SQL Dump
-- version 5.0.4
-- https://www.phpmyadmin.net/
--
-- Host: 127.0.0.1
-- Generation Time: Mar 31, 2021 at 07:04 PM
-- Server version: 10.4.17-MariaDB
-- PHP Version: 8.0.2

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `sos`
--

-- --------------------------------------------------------

--
-- Table structure for table `miasta`
--

CREATE TABLE `miasta` (
  `id` int(11) NOT NULL,
  `nazwa` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- --------------------------------------------------------

--
-- Table structure for table `ogloszenia`
--

CREATE TABLE `ogloszenia` (
  `id` int(11) NOT NULL,
  `id_nadawca` int(11) NOT NULL,
  `nadawca` varchar(255) NOT NULL,
  `temat` varchar(255) NOT NULL,
  `widok_od` datetime NOT NULL,
  `widok_do` datetime NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- --------------------------------------------------------

--
-- Table structure for table `ogloszenia_tresc`
--

CREATE TABLE `ogloszenia_tresc` (
  `id` int(11) NOT NULL,
  `id_ogloszenie` int(11) NOT NULL,
  `dane` mediumtext NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- --------------------------------------------------------

--
-- Table structure for table `uprawnienia`
--

CREATE TABLE `uprawnienia` (
  `id` int(11) NOT NULL,
  `nazwa` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- --------------------------------------------------------

--
-- Table structure for table `uzytkownicy`
--

CREATE TABLE `uzytkownicy` (
  `id` int(11) NOT NULL,
  `login` varchar(255) NOT NULL,
  `imie` varchar(255) NOT NULL,
  `nazwisko` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- --------------------------------------------------------

--
-- Table structure for table `uzytkownicy_dane`
--

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

-- --------------------------------------------------------

--
-- Table structure for table `uzytkownicy_hasla`
--

CREATE TABLE `uzytkownicy_hasla` (
  `id` int(11) NOT NULL,
  `id_uzytkownik` int(11) NOT NULL,
  `haslo` varchar(512) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- --------------------------------------------------------

--
-- Table structure for table `uzytkownicy_uprawnienia`
--

CREATE TABLE `uzytkownicy_uprawnienia` (
  `id` int(11) NOT NULL,
  `id_uzytkownik` int(11) NOT NULL,
  `id_uprawnienie` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- --------------------------------------------------------

--
-- Table structure for table `wiadomosci`
--

CREATE TABLE `wiadomosci` (
  `id` int(11) NOT NULL,
  `nadawca` varchar(255) NOT NULL,
  `temat` varchar(255) NOT NULL,
  `priorytet` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- --------------------------------------------------------

--
-- Table structure for table `wiadomosci_tresc`
--

CREATE TABLE `wiadomosci_tresc` (
  `id` int(11) NOT NULL,
  `id_wiadomosc` int(11) NOT NULL,
  `dane` mediumtext NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- --------------------------------------------------------

--
-- Table structure for table `wiadomosci_uczestnicy`
--

CREATE TABLE `wiadomosci_uczestnicy` (
  `id` int(11) NOT NULL,
  `id_wiadomosc` int(11) NOT NULL,
  `id_uczestnik` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- --------------------------------------------------------

--
-- Table structure for table `__diesel_schema_migrations`
--

CREATE TABLE `__diesel_schema_migrations` (
  `version` varchar(50) NOT NULL,
  `run_on` timestamp NOT NULL DEFAULT current_timestamp()
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- Dumping data for table `__diesel_schema_migrations`
--

INSERT INTO `__diesel_schema_migrations` (`version`, `run_on`) VALUES
('20210331123748', '2021-03-31 12:37:59'),
('20210331124012', '2021-03-31 12:47:31');

--
-- Indexes for dumped tables
--

--
-- Indexes for table `miasta`
--
ALTER TABLE `miasta`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `ogloszenia`
--
ALTER TABLE `ogloszenia`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_nadawca` (`id_nadawca`);

--
-- Indexes for table `ogloszenia_tresc`
--
ALTER TABLE `ogloszenia_tresc`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_ogloszenie` (`id_ogloszenie`);

--
-- Indexes for table `uprawnienia`
--
ALTER TABLE `uprawnienia`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `uzytkownicy`
--
ALTER TABLE `uzytkownicy`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `uzytkownicy_dane`
--
ALTER TABLE `uzytkownicy_dane`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_uzytkownik` (`id_uzytkownik`),
  ADD KEY `id_miasto` (`id_miasto`);

--
-- Indexes for table `uzytkownicy_hasla`
--
ALTER TABLE `uzytkownicy_hasla`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_uzytkownik` (`id_uzytkownik`);

--
-- Indexes for table `uzytkownicy_uprawnienia`
--
ALTER TABLE `uzytkownicy_uprawnienia`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_uzytkownik` (`id_uzytkownik`),
  ADD KEY `id_uprawnienie` (`id_uprawnienie`);

--
-- Indexes for table `wiadomosci`
--
ALTER TABLE `wiadomosci`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `wiadomosci_tresc`
--
ALTER TABLE `wiadomosci_tresc`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_wiadomosc` (`id_wiadomosc`);

--
-- Indexes for table `wiadomosci_uczestnicy`
--
ALTER TABLE `wiadomosci_uczestnicy`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_uczestnik` (`id_uczestnik`),
  ADD KEY `id_wiadomosc` (`id_wiadomosc`);

--
-- Indexes for table `__diesel_schema_migrations`
--
ALTER TABLE `__diesel_schema_migrations`
  ADD PRIMARY KEY (`version`);

--
-- AUTO_INCREMENT for dumped tables
--

--
-- AUTO_INCREMENT for table `miasta`
--
ALTER TABLE `miasta`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `ogloszenia`
--
ALTER TABLE `ogloszenia`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `ogloszenia_tresc`
--
ALTER TABLE `ogloszenia_tresc`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `uprawnienia`
--
ALTER TABLE `uprawnienia`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `uzytkownicy`
--
ALTER TABLE `uzytkownicy`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `uzytkownicy_dane`
--
ALTER TABLE `uzytkownicy_dane`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `uzytkownicy_hasla`
--
ALTER TABLE `uzytkownicy_hasla`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `uzytkownicy_uprawnienia`
--
ALTER TABLE `uzytkownicy_uprawnienia`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `wiadomosci`
--
ALTER TABLE `wiadomosci`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `wiadomosci_tresc`
--
ALTER TABLE `wiadomosci_tresc`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

--
-- AUTO_INCREMENT for table `wiadomosci_uczestnicy`
--
ALTER TABLE `wiadomosci_uczestnicy`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT;

--
-- Constraints for dumped tables
--

--
-- Constraints for table `ogloszenia`
--
ALTER TABLE `ogloszenia`
  ADD CONSTRAINT `ogloszenia_ibfk_1` FOREIGN KEY (`id_nadawca`) REFERENCES `uzytkownicy` (`id`);

--
-- Constraints for table `ogloszenia_tresc`
--
ALTER TABLE `ogloszenia_tresc`
  ADD CONSTRAINT `ogloszenia_tresc_ibfk_1` FOREIGN KEY (`id_ogloszenie`) REFERENCES `ogloszenia` (`id`);

--
-- Constraints for table `uzytkownicy_dane`
--
ALTER TABLE `uzytkownicy_dane`
  ADD CONSTRAINT `uzytkownicy_dane_ibfk_1` FOREIGN KEY (`id_uzytkownik`) REFERENCES `uzytkownicy` (`id`),
  ADD CONSTRAINT `uzytkownicy_dane_ibfk_2` FOREIGN KEY (`id_miasto`) REFERENCES `miasta` (`id`);

--
-- Constraints for table `uzytkownicy_hasla`
--
ALTER TABLE `uzytkownicy_hasla`
  ADD CONSTRAINT `uzytkownicy_hasla_ibfk_1` FOREIGN KEY (`id_uzytkownik`) REFERENCES `uzytkownicy` (`id`);

--
-- Constraints for table `uzytkownicy_uprawnienia`
--
ALTER TABLE `uzytkownicy_uprawnienia`
  ADD CONSTRAINT `uzytkownicy_uprawnienia_ibfk_1` FOREIGN KEY (`id_uzytkownik`) REFERENCES `uzytkownicy` (`id`),
  ADD CONSTRAINT `uzytkownicy_uprawnienia_ibfk_2` FOREIGN KEY (`id_uprawnienie`) REFERENCES `uprawnienia` (`id`);

--
-- Constraints for table `wiadomosci_tresc`
--
ALTER TABLE `wiadomosci_tresc`
  ADD CONSTRAINT `wiadomosci_tresc_ibfk_1` FOREIGN KEY (`id_wiadomosc`) REFERENCES `wiadomosci` (`id`);

--
-- Constraints for table `wiadomosci_uczestnicy`
--
ALTER TABLE `wiadomosci_uczestnicy`
  ADD CONSTRAINT `wiadomosci_uczestnicy_ibfk_1` FOREIGN KEY (`id_uczestnik`) REFERENCES `uprawnienia` (`id`),
  ADD CONSTRAINT `wiadomosci_uczestnicy_ibfk_2` FOREIGN KEY (`id_wiadomosc`) REFERENCES `wiadomosci` (`id`);
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
