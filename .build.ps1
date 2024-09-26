function Get-CargoMetadata {
    $cargo = "cargo.exe"
    $args = [System.Collections.ArrayList]::new()

    [void]$args.Add('metadata')
    [void]$args.Add('--no-deps')
    [void]$args.Add('--all-features')
    [void]$args.Add('--format-version=1')

    & $cargo $args | ConvertFrom-Json
}

task install {

}
