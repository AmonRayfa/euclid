# Cache file
$path = "cache.json"
$cache = Get-Content -Path $path -Raw | ConvertFrom-Json
$goal = "Abizu1"

# Integer validation
function isInt([string]$string) { # Checks if a string can be converted to an integer.
    try { [void][int]$string; return $true }
    catch { return $false }
}


# Menu function
function menu([bool]$hide=$false, [string]$message="", [string]$color="Red", [string]$prompt="") {

    if ($hide -eq $false) {
        # Clear-Host
        Write-Host ""
        Write-Host " /######$   /######  /##   /##     /## /######$  /###### /##   /##  /###### " -ForegroundColor Green
        Write-Host "| ##__  ## /##__  ##| ##  |  ##   /##/| ##__  ##|_  ##_/| ##  | ## /##__  ##" -ForegroundColor Green
        Write-Host "| ##  \ ##| ##  \ ##| ##   \  ## /##/ | ##  \ ##  | ##  | ##  | ##| ##  \__/" -ForegroundColor Green
        Write-Host "| ######$/| ##  | ##| ##    \  ####/  | ######$   | ##  | ##  | ##|  ###### " -ForegroundColor Green
        Write-Host "| ##____/ | ##  | ##| ##     \  ##/   | ##__  ##  | ##  | ##  | ## \____  ##" -ForegroundColor Green
        Write-Host "| ##      | ##  | ##| ##      | ##    | ##  \ ##  | ##  | ##  | ## /##  \ ##" -ForegroundColor Green
        Write-Host "| ##      |  ######/| ########| ##    | #######/ /######|  ######/|  ######/" -ForegroundColor Green
        Write-Host "|__/       \______/ |________/|__/    |_______/ |______/ \______/  \______/ " -ForegroundColor Green
        
        Write-Host ""
        Write-Host ""
        
        Write-Host "Display frequency:          freq=" -NoNewline
        Write-Host $($cache.freq) -ForegroundColor Blue
        Write-Host "Target:                     target=" -NoNewline
        Write-Host $($cache.target) -ForegroundColor Blue
        Write-Host "Name:                       name=" -NoNewline
        Write-Host $($cache.name) -ForegroundColor Blue

        Write-Host ""

        Write-Host "Minimum length:             min=" -NoNewline
        Write-Host $($cache.min) -ForegroundColor Magenta
        Write-Host "Maximum length:             max=" -NoNewline
        Write-Host $($cache.max) -ForegroundColor Magenta

        Write-Host ""

        Write-Host "Sequential characters:      seq=" -NoNewline
        Write-Host $($cache.seq) -ForegroundColor Cyan
        Write-Host "Same characters:            same=" -NoNewline
        Write-Host $($cache.same) -ForegroundColor Cyan
        Write-Host "Case-sensitive:             case=" -NoNewline
        Write-Host $($cache.case) -ForegroundColor Cyan

        Write-Host ""

        Write-Host "Character index:            index={" -NoNewline
        for ($k=1; $k -lt $cache.max; $k++) {
            if ($k -eq $cache.index) { Write-Host " * " -ForegroundColor Yellow -NoNewline }
            else { Write-Host " * " -NoNewline }
        }
        if ($cache.max -eq $cache.index) { Write-Host " * " -ForegroundColor Yellow -NoNewline; Write-Host "}" }
        else { Write-Host " * }" }
        Write-Host "Associated set:             set=[" -NoNewline
        Write-Host $($cache.sets.$($cache.index)) -ForegroundColor Yellow -NoNewline
        Write-Host "]"
    }
    
    Write-Host ""
    if ($message -ne "") { Write-Host $message -ForegroundColor $color; Write-Host "" }
    
    # Prompter
    Write-Host "(" -NoNewline
    Write-Host "prompt" -NoNewline -ForegroundColor Red
    Write-Host ")" -NoNewline
    Write-Host "$" -NoNewline -ForegroundColor Green
    Write-Host ": " -NoNewline
    $prompt = Read-Host
    $prompt = $prompt.Trim()

    if ($prompt -like "freq*") {
        if ($prompt -ceq "freq") {
            $hide = $true
            $message = "The frequency at which the program displays the number of attempts and skips."
            $color = "White"
        } elseif ($prompt -like "freq=*") {
            if ((isInt $prompt.Substring(5)) -eq $false -or [int]$prompt.Substring(5) -le 0) {
                $hide = $true
                $message = "ERROR - Invalid value: '$($prompt.Substring(5))'. The value must be an integer greater than 0."
                $color = "Red"
            } else {
                $script:cache.freq = [int]$prompt.Substring(5)
                $hide = $false
                $message = ""
            }
        }
    } elseif ($prompt -like "target*") {
        if ($prompt -ceq "target") {
            $hide = $true
            $message = "The target of the brute force algorithm. The value must be 'user' or 'zip'."
            $color = "White"
        } elseif ($prompt -like "target=*") {
            if ($prompt.Substring(7) -ne "user" -and $prompt.Substring(7) -ne "zip") {
                $hide = $true
                $message = "ERROR - Invalid value: '$($prompt.Substring(7))'. The value must be 'user' or 'zip'."
                $color = "Red"
            } else {
                $script:cache.target = $prompt.Substring(7)
                $hide = $false
                $message = ""
            }
        }
    } elseif ($prompt -like "name*") {
        if ($prompt -ceq "name") {
            $hide = $true
            $message = "The name of the target. The value must be a valid username or a valid path to a zip file depending on the target."
            $color = "White"
        } elseif ($prompt -like "name=*") {
            if ($prompt.Substring(5) -eq "") {
                $hide = $true
                $message = "ERROR - Invalid value: '$($prompt.Substring(5))'. The value must not be empty."
                $color = "Red"
            } elseif ($cache.target -eq "zip" -and ((Test-Path $prompt.Substring(5)) -eq $false -or $prompt.Substring(5) -like "*.zip")) {
                $hide = $true
                $message = "ERROR - Invalid value: '$($prompt.Substring(5))'. The value must be a valid path to a zip file."
                $color = "Red"
            } else {
                $script:cache.name = $prompt.Substring(5)
                $hide = $false
                $message = ""
            }
        }
    } elseif ($prompt -like "min*") {
        if ($prompt -ceq "min") {
            $hide = $true
            $message = "The minimum length of the passwords."
            $color = "White"
        } elseif ($prompt -like "min=*") {
            if ((isInt $prompt.Substring(4)) -eq $false -or [int]$prompt.Substring(4) -le 0 -or [int]$prompt.Substring(4) -gt 16) {
                $hide = $true
                $message = "ERROR - Invalid value: '$($prompt.Substring(4))'. The value must be an integer greater than 0 and less than 17."
                $color = "Red"
            } else {
                if ([int]$prompt.Substring(4) -gt $cache.max) {  $script:cache.max = [int]$prompt.Substring(4) }
                $script:cache.min = [int]$prompt.Substring(4)
                $hide = $false
                $message = ""
            }
        }
    } elseif ($prompt -like "max*") {
        if ($prompt -ceq "max") {
            $hide = $true
            $message = "The maximum length of the passwords."
            $color = "White"
        } elseif ($prompt -like "max=*") {
            if ((isInt $prompt.Substring(4)) -eq $false -or [int]$prompt.Substring(4) -le 0 -or [int]$prompt.Substring(4) -gt 16) {
                $hide = $true
                $message = "ERROR - Invalid value: '$($prompt.Substring(4))'. The value must be an integer greater than 0 and less than 17."
                $color = "Red"
            } else {
                if ([int]$prompt.Substring(4) -lt $cache.min) { $script:cache.min = [int]$prompt.Substring(4) }
                if ([int]$prompt.Substring(4) -lt $cache.index) { $script:cache.index = [int]$prompt.Substring(4) }
                $script:cache.max = [int]$prompt.Substring(4)
                $hide = $false
                $message = ""
            }
        }
    } elseif ($prompt -like "seq*") {
        if ($prompt -ceq "seq") {
            $hide = $true
            $message = "The maximum number of sequential characters in every attempted password."
            $color = "White"
        } elseif ($prompt -like "seq=*") {
            if ((isInt $prompt.Substring(4)) -eq $false -or [int]$prompt.Substring(4) -lt 0) {
                $hide = $true
                $message = "ERROR - Invalid value: '$($prompt.Substring(4))'. The value must be an integer greater than or equal to 0."
                $color = "Red"
            } else {
                $script:cache.seq = [int]$prompt.Substring(4)
                $hide = $false
                $message = ""
            }
        }
    } elseif ($prompt -like "same*") {
        if ($prompt -ceq "same") {
            $hide = $true
            $message = "The maximum number of same characters in every attempted password."
            $color = "White"
        } elseif ($prompt -like "same=*") {
            if ((isInt $prompt.Substring(5)) -eq $false -or [int]$prompt.Substring(5) -lt 0) {
                $hide = $true
                $message = "ERROR - Invalid value: '$($prompt.Substring(5))'. The value must be an integer greater than or equal to 0."
                $color = "Red"
            } else {
                $script:cache.same = [int]$prompt.Substring(5)
                $hide = $false
                $message = ""
            }
        }
    } elseif ($prompt -like "case*") {
        if ($prompt -ceq "case") {
            $hide = $true
            $message = "Whether or not the password is case-sensitive."
            $color = "White"
        } elseif ($prompt -like "case=*") {
            if ($prompt.Substring(5) -ne "True" -and $prompt.Substring(5) -ne "False" -and $prompt.Substring(5) -ne "true" -and $prompt.Substring(5) -ne "false") {
                $hide = $true
                $message = "ERROR - Invalid value: '$($prompt.Substring(5))'. The value must be 'True', 'False', 'true' or 'false'."
                $color = "Red"
            } else {
                $script:cache.case = [System.Convert]::ToBoolean($prompt.Substring(5))
                $hide = $false
                $message = ""
            }
        }
    } elseif ($prompt -like "index*") {
        if ($prompt -ceq "index") {
            $hide = $true
            $message = "The index of the current character."
            $color = "White"
        } elseif ($prompt -like "index=*") {
            if ((isInt $prompt.Substring(6)) -eq $false -or [int]$prompt.Substring(6) -le 0 -or [int]$prompt.Substring(6) -gt $cache.max) {
                $hide = $true
                $message = "ERROR - Invalid value: '$($prompt.Substring(6))'. The value must be an integer greater than 0 and less than or equal to the maximum length of the passwords."
                $color = "Red"
            } else {
                $script:cache.index = [int]$prompt.Substring(6)
                $hide = $false
                $message = ""
            }
        }
    } elseif ($prompt -like "set*") {
        if ($prompt -ceq "set") {
            $hide = $true
            $message = "The character set of the current character."
            $color = "White"
        } elseif ($prompt -like "set=*") {
            if ($prompt.Substring(4) -eq "") {
                $hide = $true
                $message = "ERROR - Invalid value: '$($prompt.Substring(4))'. The value must be a non-empty string."
                $color = "Red"
            } elseif ($prompt.Substring(4) -match "[^\x00-\x7F]") {
                $hide = $true
                $message = "ERROR - Invalid value: '$($prompt.Substring(4))'. The value must be a string containing only ASCII characters."
                $color = "Red"
            } else {
                $script:cache.sets.$($cache.index) = $prompt.Substring(4)
                $hide = $false
                $message = ""
            }
        }
    } elseif ($prompt -like "help") {
        $hide = $true
        $message = "Type 'option=<value>' to set a value for an option.`nType the name of an option to get more information about it.`nType 'exit', 'quit', 'q', 'e', 'Exit', 'Quit', 'Q' or 'E' to exit the program.`nPush 'Enter' to start the brute force algorithm."
        $color = "White"
    } else {
        if ($prompt -match "=") { $message = "ERROR - Invalid option: '$($prompt.Substring(0, $prompt.IndexOf("=")))'. Type 'help' for more information." }
        else { $message = "ERROR - Invalid command: '$prompt'. Type 'help' for more information." }
        $hide = $true
        $color = "Red"
    }

    if ($prompt -eq "" -or $prompt -eq "exit" -or $prompt -eq "quit" -or $prompt -eq "q" -or $prompt -eq "e" -or $prompt -eq "Exit" -or $prompt -eq "Quit" -or $prompt -eq "Q" -or $prompt -eq "E") {
        if ($prompt -eq "") {
            if ($cache.target -eq "zip" -and ((Test-Path $cache.name) -eq $false -or $cache.name -like "*.zip")) {
                $message = "ERROR - Invalid name value: '$($cache.name)'. The value must be a valid path to a zip file."
                menu $true $message "Red"
            } else {
                $script:cache | ConvertTo-Json -Depth 100 | Set-Content -Path $path
                Write-Host ""
                Write-Host "START - Brute force algorithm" -ForegroundColor Green
                Write-Host ""
            }
        } else {
            $script:cache | ConvertTo-Json -Depth 100 | Set-Content -Path $path
            Write-Host ""
            Write-Host "END - User exit" -ForegroundColor Red
            Write-Host ""
            exit
        }
    } else {
        menu $hide $message $color
    }

}


# Brute force algorithm
function cracker {
    # TODO: Make a multi-threaded version of the program.
    #? For some strange reason, this version of the program and it's Rust equivalent don't produce the same final number of attempts and skips.
    $attempted = 1
    $skipped = 1
    $depth = $cache.min
    $counters = @()
    for ($i=1; $i -le $cache.max; $i++) { $counters += $cache.sets.$i.Length }
    $credentials
    $result = $false

    #* We declare these variables outside the loop to avoid creating them every iteration.
    $password
    $valid
    $seq
    $same

    while ($depth -le $cache.max) {
        $password = $cache.sets."1"[0]
        $valid = $true
        $seq = 0
        
        if ($depth -ge 2) {
            for ($j=2; $j -le $depth; $j++) {
                $same = 0
                if ($cache.case) {
                    if ($password[-1] -ceq $cache.sets.$j[0]) {
                        $seq++
                        if ($seq -gt $cache.seq) { $valid = $false; break }
                    }
                    else { $seq = 0 }
                    for ($k=0; $k -lt $password.Length; $k++) { #? For some reason, when I change `$password.Length` to `$depth`, the algorithm gets significantly slower.
                        if ($password[$k] -ceq $cache.sets.$j[0]) {
                            $same++
                            if ($same -gt $cache.same) { $valid = $false; break }
                        }
                    }
                } else {
                    if ($password[-1] -eq $cache.sets.$j[0]) {
                        $seq++
                        if ($seq -gt $cache.seq) { $valid = $false; break }
                    }
                    else { $seq = 0 }
                    for ($k=0; $k -lt $password.Length; $k++) {
                        if ($password[$k] -eq $cache.sets.$j[0]) {
                            $same++
                            if ($same -gt $cache.same) { $valid = $false; break }
                        }
                    }
                }
                if ($valid) { $password += $cache.sets.$j[0] }
            }
        }

        if ($valid) {
            if ($goal -ceq $password) { # Test
                Write-Host "($attempted) {$skipped} [$password] --> SUCCESS" -ForegroundColor Green
                $result = $true
                break
            } else  {
                if ($attempted % $cache.freq -eq 0) { Write-Host "($attempted) {$skipped} [$password] --> FAILURE" }
                $attempted++
            }
            <#
            if ($cache.target -eq "user") {
                $password = ConvertTo-SecureString -String $password -AsPlainText -Force
                $credentials = New-Object System.Management.Automation.PSCredential($cache.name, $password)
                try {
                    $result = Test-Connection -ComputerName $cache.target -Credential $credentials -Count 1 -ErrorAction Stop
                    if ($result) { break }
                } catch { }
            } elseif ($cache.target -eq "zip") {
                try {
                    $result = Expand-Archive -Path $cache.name -DestinationPath "../" -Force -ErrorAction Stop
                    if ($result) { break }
                } catch { }
            }
            #>
        } else {
            if ($skipped % $cache.freq -eq 0) { Write-Host "($attempted) {$skipped} [$("#"*$depth)] --> SKIPPED" }
            $skipped++
        }

        for ($i=$depth; $i -ge 1; $i--) {
            if ($cache.sets.$i.Length -gt 1) { $script:cache.sets.$i = $cache.sets.$i.Substring(1) + $cache.sets.$i[0] }
            $counters[$i-1] -= 1
            if ($i -ne 1 -and $counters[$i-1] -eq 0) { $counters[$i-1] = $cache.sets.$i.Length }
            else { break }
        }
        if ($counters[0] -eq 0) { $depth++; $counters[0] = $cache.sets."1".Length }
    }

    if ($result) {
        Write-Host ""
        Write-Host "END - Password found" -ForegroundColor Red
        Write-Host ""
    } else {
        Write-Host ""
        Write-Host "END - No password found" -ForegroundColor Red
        Write-Host ""
    }
}


# Main
menu
cracker
