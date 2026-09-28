function grab_path
{
    write-Host -ForegroundColor Green "Gimmie the file path"
    $file = Read-Host
    write-host -ForegroundColor Green "Key name?"
    $keyname = Read-Host

    return $file, $keyname
}


# Query the run and runonce registries. Everyone and their mother monitors these so if there's an application that can be replaced, that'd be preferred over adding something.
Write-Host -ForegroundColor Green "Starting to query the run registry."
Get-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"

Write-Host -ForegroundColor Green "Starting to query the runonce registry."
Get-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\RunOnce"

write-warning "Everyone and their mother monitors these so if there's an application that can be replaced, that'd be preferred over adding something."

write-host -ForegroundColor Yellow "Would you like to add to the run, runonce, both, or quit?"
$useranswer = Read-Host

if ($useranswer -eq "run")
{
    $file, $keyname = grab_path
    New-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" -Name $keyname -Value $file
    Write-Host -ForegroundColor Green "Successfully wrote $file to the run registry with the name $keyname."
}
elseif ($useranswer -eq "runonce")
{
    $file, $keyname = grab_path
    New-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\RunOnce" -Name $keyname -Value $file
    Write-Host -ForegroundColor Green "Successfully wrote $file to the runonce registry with the name $keyname."
}
elseif ($useranswer -eq "both")
{
    $file, $keyname = grab_path
    New-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" -Name $keyname -Value $file
    New-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\RunOnce" -Name $keyname -Value $file
    Write-Host -ForegroundColor Green "Successfully wrote $file to both registries with the name $keyname."
}
else
{
    exit
}
