ALTER TABLE estabelecimentos 
    DROP FOREIGN KEY FK_EstabEmp,
    DROP FOREIGN KEY FK_EstabMotivCad,
    DROP FOREIGN KEY FK_EstabPais,
    DROP FOREIGN KEY FK_EstabCnaePrinc,
    DROP FOREIGN KEY FK_EstabMunic;
ALTER TABLE empresas
    DROP FOREIGN KEY FK_EmpNatJur,
    DROP FOREIGN KEY FK_EmpQualResp;