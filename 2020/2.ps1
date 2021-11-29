#!/usr/bin/env pwsh
# Part I.
Get-Content ./2.input |
    Where-Object {
        $policy, $password = $_.Split(': ')
        $crange, $char = $policy.Split(' ')
        [int]$cmin, [int]$cmax = $crange.Split('-')
        $i = 0
        foreach ($c in $password.ToCharArray()) {
            if ($c -eq $char) {
                $i += 1
            }
        }
        return ($i -ge $cmin) -and ($i -le $cmax)
    } |
    Measure-Object | Select-Object -Property Count | Write-Output

# Part II.
Get-Content ./2.input |
    Where-Object {
        $policy, $password = $_.Split(': ')
        $crange, $char = $policy.Split(' ')
        [int]$pa, [int]$pb = $crange.Split('-')
        [int]$r = ($password[$pa-1] -eq $char) + ($password[$pb-1] -eq $char)
        return $r -eq 1
    } |
    Measure-Object | Select-Object -Property Count | Write-Output
