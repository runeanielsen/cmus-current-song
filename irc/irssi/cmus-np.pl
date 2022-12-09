use strict;
use Irssi;

sub np
{
    my($data,$server,$witem) = @_; #loads irssi env.
    my $raw_cmus_current_song = `cmus-current-song`;
    my $cmus_current_song = "Now Playing: $raw_cmus_current_song";
    #prints to the channel
    if ($witem && ($witem->{type} eq "CHANNEL" || $witem->{type} eq "QUERY")) 
    {
        $witem->command("MSG ".$witem->{name}." $cmus_current_song");
    }
    else 
    {
        Irssi::print("You're not in a channel.");
    }
}

sub help
{
    print '
cmus(1) now-playing script
--------------------------

Use /np to display the song currently playing in C*Mus.
Use /nphelp to print this message.
Install cmus-current-song and test it against your C*Mus process to make this work;
no other weird dependencies.

Enjoy!';
}

#now to bind this shit to actual irssi commands
Irssi::command_bind np        => \&np;
Irssi::command_bind 'nphelp' => \&help;

#notify user on successful load
Irssi::print "cmus-np.pl loaded - /np help for help";



