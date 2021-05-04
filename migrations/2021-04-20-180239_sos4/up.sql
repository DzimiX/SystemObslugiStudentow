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
  `dane` mediumtext NOT NULL,
  `widok_od` datetime NOT NULL,
  `widok_do` datetime NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- --------------------------------------------------------

--
-- Table structure for table `tokeny`
--

CREATE TABLE `tokeny` (
  `id` int(11) NOT NULL,
  `id_uzytkownik` int(11) NOT NULL,
  `id_uprawnienie` int(11) NOT NULL,
  `token` varchar(512) NOT NULL,
  `data` bigint(8) NOT NULL
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
  `id_uzytkownik` int(11) NOT NULL,
  `temat` varchar(255) NOT NULL,
  `data` bigint(8) NOT NULL,
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
-- Indexes for table `tokeny`
--
ALTER TABLE `tokeny`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `token` (`token`),
  ADD KEY `id_uzytkownik` (`id_uzytkownik`),
  ADD KEY `id_uprawnienie` (`id_uprawnienie`);

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
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_uzytkownik` (`id_uzytkownik`);


--
-- Indexes for table `wiadomosci_uczestnicy`
--
ALTER TABLE `wiadomosci_uczestnicy`
  ADD PRIMARY KEY (`id`),
  ADD KEY `id_wiadomosc` (`id_wiadomosc`),
  ADD KEY `wiadomosci_uczestnicy_ibfk_1` (`id_uczestnik`);

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
-- AUTO_INCREMENT for table `tokeny`
--
ALTER TABLE `tokeny`
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
-- Constraints for table `tokeny`
--
ALTER TABLE `tokeny`
  ADD CONSTRAINT `tokeny_ibfk_1` FOREIGN KEY (`id_uzytkownik`) REFERENCES `uzytkownicy` (`id`),
  ADD CONSTRAINT `tokeny_ibfk_2` FOREIGN KEY (`id_uprawnienie`) REFERENCES `uprawnienia` (`id`);

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



ALTER TABLE `wiadomosci`
  ADD CONSTRAINT `wiadomosci_ibfk_1` FOREIGN KEY (`id_uzytkownik`) REFERENCES `uzytkownicy` (`id`);

--
-- Constraints for table `wiadomosci_uczestnicy`
--
ALTER TABLE `wiadomosci_uczestnicy`
  ADD CONSTRAINT `wiadomosci_uczestnicy_ibfk_1` FOREIGN KEY (`id_uczestnik`) REFERENCES `uzytkownicy` (`id`),
  ADD CONSTRAINT `wiadomosci_uczestnicy_ibfk_2` FOREIGN KEY (`id_wiadomosc`) REFERENCES `wiadomosci` (`id`);


INSERT INTO `uprawnienia` (`id`, `nazwa`) VALUES
  (1, 'Użytkownik'),
  (2, 'Student'),
  (3, 'Prowadzący'),
  (4, 'Pracownik'),
  (5, 'Administrator');

INSERT INTO `uzytkownicy` (`id`, `login`, `imie`, `nazwisko`) VALUES
  (1, 'anowak', 'Adam', 'Nowak'),
  (2, 'jkowalski', 'Jan', 'Kowalski');

INSERT INTO `uzytkownicy_hasla` (`id`, `id_uzytkownik`, `haslo`) VALUES
  (3, 1, '$2b$08$riVVp/djalI7Ei2HAiTpFeQyYE6Fr4ijnVNaKZW2.d4i2Xhx83y5q'),
  (5, 2, '$2b$08$vQ.Q/XWK8Rmoxg4JmdiJTuXgXtPRHRsDJWoybPgL.RHXR24uhXv0u');

INSERT INTO `uzytkownicy_uprawnienia` (`id`, `id_uzytkownik`, `id_uprawnienie`) VALUES
  (1, 1, 5),
  (2, 2, 4);

INSERT INTO `tokeny` (`id`, `id_uzytkownik`, `id_uprawnienie`, `token`) VALUES
  (3, 2, 4, '5ybvJkeimJ%&g((po#ajfArJ1nmcHhfTGQReEn#f)%l0Wn&9flXUXj7QzXjPx@nRN@(CWh3N4zIguw%I2AodFY9l8XGx^sP&DXUENeHLkfXeHz7v2KD&bGzr16n&e~RjhxOk&5sbO5HPwfOAskdU$yDhoOW&%dALe(3em)HQ#LfDhd1Mv7yAytD08OYT&!ZE)%b3pjMDzuL&o3KZuf)N^j7l3ln0&MaIu12xL9p%vk8Aj)!NA8^Oj0OP%TU$ibPmIAbX#un^nnt56RC7fuA@9az%SyvwnoSHYW!kiNPiedUJI!E3S~e0(99Rq!wh0m!7f#^WTL%BA*^WoxU0hIytQGsrYVh#ml*8yz!F8KlNycT3ywp~uU4BER^ZqpjUSYegxFdTK^WWNJ(oHi&^mlmwAWuGN^HdlZ(DQTcDC4&0YIkHAazG*lJJHchhHZaNCK56snsR5J8jk4cVA#)Np%mvv6P*rO9IVM!dmVaw~6hLc(DKu46U@Xe5epK3xy*@WEi^'),
  (4, 1, 5, 'f6*E%E^%L%GpJ45OW$#e~*3VBto~o^psc%utIKQqD&z$HQq)qMLQ%m8CKbt66tY%ofgavySw)~7Hdcajk32I6&M7wS&50mKQGOa^jR~^YWJDf4aUxtqj~1DDdtuqVCo(ZC&6BkQKo0@gurdu(3*C6lCeyO~yKRj*hBs1H1waa@uh4zgi9W#HFGgDyjx~)(fW%h(!%GvokZ(E)qVT1%*FvAY98i1##Kgbc8y6rrAwL*u5s$honqyo!J6H3bgrQGl)JjIw1nt^hKTQV8qq^FxCiH*rGcS^6SvH&&je&)UMGfDGLv9@B#7D**yzEnl43l)qgYMcNQH#kga(U@tj~V6a$t(smOib92fLb8JiZ3RoR8Lz^^3G~wh9J^G2nm*2@%cOs^w*&YveGQM1Nmem2ls2I5d~h18#40dcm7FgSI#M@AIwUc4Ii$dqcm0U76SA2P(#Iva)lQXhrRa*Zvc!aMcvUHN#*2nGRfj9cXCLd815a^q^qiNTzxFKo)IV@QcH4)Zf');