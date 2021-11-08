"""This sub-module offers methods to automatically retrieve the graphs from KGOBO repository."""

from .aeo import AEO
from .agro import AGRO
from .aism import AISM
from .amphx import AMPHX
from .apo import APO
from .apollo_sv import APOLLO_SV
from .aro import ARO
from .bco import BCO
from .bfo import BFO
from .bspo import BSPO
from .bto import BTO
from .cdao import CDAO
from .cdno import CDNO
from .ceph import CEPH
from .chebi import CHEBI
from .cheminf import CHEMINF
from .chiro import CHIRO
from .chmo import CHMO
from .cido import CIDO
from .cio import CIO
from .cl import CL
from .clao import CLAO
from .clo import CLO
from .clyh import CLYH
from .cmo import CMO
from .cob import COB
from .cro import CRO
from .cteno import CTENO
from .cto import CTO
from .cvdo import CVDO
from .ddanat import DDANAT
from .ddpheno import DDPHENO
from .dideo import DIDEO
from .doid import DOID
from .dpo import DPO
from .dron import DRON
from .duo import DUO
from .ecao import ECAO
from .eco import ECO
from .ecocore import ECOCORE
from .ecto import ECTO
from .ehdaa2 import EHDAA2
from .emapa import EMAPA
from .envo import ENVO
from .ero import ERO
from .eupath import EUPATH
from .exo import EXO
from .fao import FAO
from .fbbi import FBBI
from .fbbt import FBBT
from .fbcv import FBCV
from .fbdv import FBDV
from .fix import FIX
from .flopo import FLOPO
from .fma import FMA
from .foodon import FOODON
from .fovt import FOVT
from .fypo import FYPO
from .gaz import GAZ
from .gecko import GECKO
from .genepio import GENEPIO
from .geno import GENO
from .geo import GEO
from .gno import GNO
from .go import GO
from .gsso import GSSO
from .hancestro import HANCESTRO
from .hao import HAO
from .hom import HOM
from .hp import HP
from .hsapdv import HSAPDV
from .hso import HSO
from .htn import HTN
from .iao import IAO
from .iceo import ICEO
from .ico import ICO
from .ido import IDO
from .idomal import IDOMAL
from .ino import INO
from .kisao import KISAO
from .labo import LABO
from .ma import MA
from .mamo import MAMO
from .maxo import MAXO
from .mco import MCO
from .mf import MF
from .mfmo import MFMO
from .mfoem import MFOEM
from .mfomd import MFOMD
from .mi import MI
from .miapa import MIAPA
from .miro import MIRO
from .mmo import MMO
from .mmusdv import MMUSDV
from .mod import MOD
from .mondo import MONDO
from .mop import MOP
from .mp import MP
from .mpath import MPATH
from .mpio import MPIO
from .mro import MRO
from .ms import MS
from .nbo import NBO
from .ncbitaxon import NCBITAXON
from .ncit import NCIT
from .ncro import NCRO
from .nomen import NOMEN
from .oae import OAE
from .oarcs import OARCS
from .oba import OBA
from .obcs import OBCS
from .obi import OBI
from .obib import OBIB
from .ogg import OGG
from .ogms import OGMS
from .ogsf import OGSF
from .ohd import OHD
from .ohmi import OHMI
from .ohpi import OHPI
from .olatdv import OLATDV
from .omit import OMIT
from .omo import OMO
from .omp import OMP
from .omrse import OMRSE
from .one import ONE
from .ons import ONS
from .ontoneo import ONTONEO
from .oostt import OOSTT
from .opl import OPL
from .opmi import OPMI
from .ornaseq import ORNASEQ
from .ovae import OVAE
from .pato import PATO
from .pco import PCO
from .pdro import PDRO
from .pdumdv import PDUMDV
from .peco import PECO
from .phipo import PHIPO
from .plana import PLANA
from .planp import PLANP
from .po import PO
from .poro import PORO
from .ppo import PPO
from .pr import PR
from .psdo import PSDO
from .pso import PSO
from .pw import PW
from .rbo import RBO
from .rex import REX
from .ro import RO
from .rs import RS
from .rxno import RXNO
from .sbo import SBO
from .scdo import SCDO
from .sepio import SEPIO
from .sibo import SIBO
from .so import SO
from .spd import SPD
from .stato import STATO
from .swo import SWO
from .symp import SYMP
from .tads import TADS
from .taxrank import TAXRANK
from .tgma import TGMA
from .to import TO
from .trans import TRANS
from .tto import TTO
from .txpo import TXPO
from .uberon import UBERON
from .uo import UO
from .upa import UPA
from .upheno import UPHENO
from .vario import VARIO
from .vo import VO
from .vt import VT
from .vto import VTO
from .wbbt import WBBT
from .wbls import WBLS
from .wbphenotype import WBPHENOTYPE
from .xao import XAO
from .xco import XCO
from .xlmod import XLMOD
from .xpo import XPO
from .zeco import ZECO
from .zfa import ZFA
from .zfs import ZFS
from .zp import ZP

__all__ = [
	"AEO", "AGRO", "AISM", "AMPHX", "APO", "APOLLO_SV", "ARO", "BCO", "BFO",
	"BSPO", "BTO", "CDAO", "CDNO", "CEPH", "CHEBI", "CHEMINF", "CHIRO", "CHMO",
	"CIDO", "CIO", "CL", "CLAO", "CLO", "CLYH", "CMO", "COB", "CRO", "CTENO",
	"CTO", "CVDO", "DDANAT", "DDPHENO", "DIDEO", "DOID", "DPO", "DRON", "DUO",
	"ECAO", "ECO", "ECOCORE", "ECTO", "EHDAA2", "EMAPA", "ENVO", "ERO", "EUPATH",
	"EXO", "FAO", "FBBI", "FBBT", "FBCV", "FBDV", "FIX", "FLOPO", "FMA", "FOODON",
	"FOVT", "FYPO", "GAZ", "GECKO", "GENEPIO", "GENO", "GEO", "GNO", "GO",
	"GSSO", "HANCESTRO", "HAO", "HOM", "HP", "HSAPDV", "HSO", "HTN", "IAO",
	"ICEO", "ICO", "IDO", "IDOMAL", "INO", "KISAO", "LABO", "MA", "MAMO", "MAXO",
	"MCO", "MF", "MFMO", "MFOEM", "MFOMD", "MI", "MIAPA", "MIRO", "MMO", "MMUSDV",
	"MOD", "MONDO", "MOP", "MP", "MPATH", "MPIO", "MRO", "MS", "NBO", "NCBITAXON",
	"NCIT", "NCRO", "NOMEN", "OAE", "OARCS", "OBA", "OBCS", "OBI", "OBIB",
	"OGG", "OGMS", "OGSF", "OHD", "OHMI", "OHPI", "OLATDV", "OMIT", "OMO",
	"OMP", "OMRSE", "ONE", "ONS", "ONTONEO", "OOSTT", "OPL", "OPMI", "ORNASEQ",
	"OVAE", "PATO", "PCO", "PDRO", "PDUMDV", "PECO", "PHIPO", "PLANA", "PLANP",
	"PO", "PORO", "PPO", "PR", "PSDO", "PSO", "PW", "RBO", "REX", "RO", "RS",
	"RXNO", "SBO", "SCDO", "SEPIO", "SIBO", "SO", "SPD", "STATO", "SWO", "SYMP",
	"TADS", "TAXRANK", "TGMA", "TO", "TRANS", "TTO", "TXPO", "UBERON", "UO",
	"UPA", "UPHENO", "VARIO", "VO", "VT", "VTO", "WBBT", "WBLS", "WBPHENOTYPE",
	"XAO", "XCO", "XLMOD", "XPO", "ZECO", "ZFA", "ZFS", "ZP",
]