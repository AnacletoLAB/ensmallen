"""This sub-module offers methods to automatically retrieve the graphs from KGOBO repository."""

from .mod import MOD
from .fbbt import FBBT
from .bto import BTO
from .chmo import CHMO
from .oba import OBA
from .ogsf import OGSF
from .mco import MCO
from .opmi import OPMI
from .fbdv import FBDV
from .ceph import CEPH
from .mpath import MPATH
from .spd import SPD
from .omit import OMIT
from .vt import VT
from .ehdaa2 import EHDAA2
from .wbls import WBLS
from .rxno import RXNO
from .omp import OMP
from .ero import ERO
from .gno import GNO
from .xco import XCO
from .amphx import AMPHX
from .clyh import CLYH
from .oostt import OOSTT
from .ncro import NCRO
from .iao import IAO
from .geo import GEO
from .exo import EXO
from .swo import SWO
from .obcs import OBCS
from .symp import SYMP
from .taxrank import TAXRANK
from .apo import APO
from .clo import CLO
from .cmo import CMO
from .hso import HSO
from .obi import OBI
from .cdao import CDAO
from .cro import CRO
from .cheminf import CHEMINF
from .mp import MP
from .duo import DUO
from .labo import LABO
from .olatdv import OLATDV
from .mpio import MPIO
from .chebi import CHEBI
from .aeo import AEO
from .geno import GENO
from .sbo import SBO
from .to import TO
from .uo import UO
from .mop import MOP
from .chiro import CHIRO
from .ogms import OGMS
from .ncbitaxon import NCBITAXON
from .pw import PW
from .fovt import FOVT
from .xpo import XPO
from .zfs import ZFS
from .rs import RS
from .omo import OMO
from .fix import FIX
from .mamo import MAMO
from .vto import VTO
from .uberon import UBERON
from .mfomd import MFOMD
from .bfo import BFO
from .htn import HTN
from .poro import PORO
from .aism import AISM
from .wbbt import WBBT
from .hao import HAO
from .so import SO
from .mondo import MONDO
from .ddpheno import DDPHENO
from .idomal import IDOMAL
from .fbcv import FBCV
from .trans import TRANS
from .psdo import PSDO
from .scdo import SCDO
from .ontoneo import ONTONEO
from .dron import DRON
from .rbo import RBO
from .ncit import NCIT
from .fma import FMA
from .rex import REX
from .cob import COB
from .sibo import SIBO
from .pdro import PDRO
from .ogg import OGG
from .hancestro import HANCESTRO
from .go import GO
from .mf import MF
from .plana import PLANA
from .oae import OAE
from .mmusdv import MMUSDV
from .ms import MS
from .apollo_sv import APOLLO_SV
from .hsapdv import HSAPDV
from .miro import MIRO
from .emapa import EMAPA
from .gecko import GECKO
from .genepio import GENEPIO
from .tads import TADS
from .fao import FAO
from .cvdo import CVDO
from .ecao import ECAO
from .opl import OPL
from .tgma import TGMA
from .bco import BCO
from .ico import ICO
from .zeco import ZECO
from .pdumdv import PDUMDV
from .aro import ARO
from .oarcs import OARCS
from .planp import PLANP
from .doid import DOID
from .omrse import OMRSE
from .ppo import PPO
from .ovae import OVAE
from .zp import ZP
from .stato import STATO
from .one import ONE
from .ecto import ECTO
from .xao import XAO
from .miapa import MIAPA
from .mi import MI
from .ecocore import ECOCORE
from .mmo import MMO
from .eupath import EUPATH
from .obib import OBIB
from .ido import IDO
from .sepio import SEPIO
from .tto import TTO
from .pr import PR
from .nbo import NBO
from .wbphenotype import WBPHENOTYPE
from .peco import PECO
from .cio import CIO
from .clao import CLAO
from .upa import UPA
from .zfa import ZFA
from .ma import MA
from .po import PO
from .cdno import CDNO
from .ons import ONS
from .ohd import OHD
from .vario import VARIO
from .agro import AGRO
from .dideo import DIDEO
from .txpo import TXPO
from .pato import PATO
from .hom import HOM
from .eco import ECO
from .ddanat import DDANAT
from .bspo import BSPO
from .mro import MRO
from .pco import PCO
from .ornaseq import ORNASEQ
from .hp import HP
from .dpo import DPO
from .cl import CL
from .mfoem import MFOEM

__all__ = [
	"MOD", "FBBT", "BTO", "CHMO", "OBA", "OGSF", "MCO", "OPMI", "FBDV", "CEPH",
	"MPATH", "SPD", "OMIT", "VT", "EHDAA2", "WBLS", "RXNO", "OMP", "ERO", "GNO",
	"XCO", "AMPHX", "CLYH", "OOSTT", "NCRO", "IAO", "GEO", "EXO", "SWO", "OBCS",
	"SYMP", "TAXRANK", "APO", "CLO", "CMO", "HSO", "OBI", "CDAO", "CRO", "CHEMINF",
	"MP", "DUO", "LABO", "OLATDV", "MPIO", "CHEBI", "AEO", "GENO", "SBO", "TO",
	"UO", "MOP", "CHIRO", "OGMS", "NCBITAXON", "PW", "FOVT", "XPO", "ZFS",
	"RS", "OMO", "FIX", "MAMO", "VTO", "UBERON", "MFOMD", "BFO", "HTN", "PORO",
	"AISM", "WBBT", "HAO", "SO", "MONDO", "DDPHENO", "IDOMAL", "FBCV", "TRANS",
	"PSDO", "SCDO", "ONTONEO", "DRON", "RBO", "NCIT", "FMA", "REX", "COB",
	"SIBO", "PDRO", "OGG", "HANCESTRO", "GO", "MF", "PLANA", "OAE", "MMUSDV",
	"MS", "APOLLO_SV", "HSAPDV", "MIRO", "EMAPA", "GECKO", "GENEPIO", "TADS",
	"FAO", "CVDO", "ECAO", "OPL", "TGMA", "BCO", "ICO", "ZECO", "PDUMDV", "ARO",
	"OARCS", "PLANP", "DOID", "OMRSE", "PPO", "OVAE", "ZP", "STATO", "ONE",
	"ECTO", "XAO", "MIAPA", "MI", "ECOCORE", "MMO", "EUPATH", "OBIB", "IDO",
	"SEPIO", "TTO", "PR", "NBO", "WBPHENOTYPE", "PECO", "CIO", "CLAO", "UPA",
	"ZFA", "MA", "PO", "CDNO", "ONS", "OHD", "VARIO", "AGRO", "DIDEO", "TXPO",
	"PATO", "HOM", "ECO", "DDANAT", "BSPO", "MRO", "PCO", "ORNASEQ", "HP",
	"DPO", "CL", "MFOEM",
]